**************************
Parsing Existing PDF Files
**************************

If you already have some PDF documents on hand that you would like to parse, you do not need to run the full
extraction pipeline. In this example, we will show you how to apply just the ordinance extraction portion to
your existing documents. (For a basic example on just downloading the documents without extraction, see the
`ELM tutorial <https://github.com/NREL/elm/blob/main/examples/web_scraping_pipeline/example_scrape_wiki.ipynb>`_).

We will start by going over some components that we will need for extraction and the we'll put them all together
into a final script.


Extraction Components
=====================

Document Class
--------------
The first important component we will discuss is the :class:`elm.web.document.PDFDocument` class. This class
(and the corresponding :class:`elm.web.document.HTMLDocument`) are used to represent individual documents that
are being parsed for ordinance information. They contain important information about the document like the
text contents as well as some helpful processing routines (i.e. clean out headers, etc.). These classes also
have an ``attrs`` attribute that tracks metadata associated with the document. This attribute is used by
``COMPASS`` to track things like the documents' source URL, the document date, the ordinance text, the
structured ordinance values, etc. Many of the ``COMPASS`` functions require a document as an input.

To create a document class that we can use for ordinance extraction, we can use the following code:

.. code-block:: python

    from elm.web.document import PDFDocument

    doc = PDFDocument.from_file("Decatur County, Indiana.pdf")

If you check the ``doc.attrs`` attribute at this point, it will be empty. As we call various ``COMPASS``
routines, the dictionary will be populated.


LLM Caller Arguments
--------------------
The next component we will discuss is the :class:`~compass.llm.calling.LLMCallerArgs` class. This class
is responsible for tracking our configuration for calling an LLM. We can instantiate the class like so:

.. code-block:: python

    from compass.llm import LLMCallerArgs

    caller_args = LLMCallerArgs(
        name="gpt-4o-mini",
        llm_call_kwargs={"temperature": 0},
        llm_service_rate_limit=500_000,
        text_splitter_chunk_size=10_000,
        text_splitter_chunk_overlap=500,
        client_type="azure",
        client_kwargs={
            "api_key": "<your API key>",
            "api_version": "<your API version>",
            "azure_endpoint": "<your API endpoint>",
        },
    )


In this example, we configured the ``gpt-4o-mini`` model to be called with a ``temperature`` parameter of ``0``.
We also specified that the rate limit for this model is 500k tokens per minute. We also specified that the text
splitter instance for this model should have chunks of 10k tokens each, with 500 tokens of overlap. Finally, we
specified that this is an Azure OpenAI model, and gave ourselves the option of specifying the API key, endpoint,
and version in the code directly (in practice, it's best to set these as environment variables, as we'll see in
`Executing the Script`_).

You can create multiple ``LLMCallerArgs`` instances and have them be responsible for different parts of
of the extraction process. In this example, we will use only one LLM for the entire processing pipeline.


COMPASS Services
----------------
``COMPASS`` Services are utility classes that can run asynchronously while your script is running. You can
call these services using a class method, which allows you to use them at any point in your code without
having to pass around object instances. Behind the scenes, these services work by having a queue that tracks
all of the processing "requests" that have been made and processing them assuming the required resources are
available (e.g. we have not hit a rate limit for querying an LLM).

The price you have to pay for these services is to make sure their queue is initialized and that each service
is watching the queue and ready to process it. In practice, this means you need to use the
:class:`~compass.services.provider.RunningAsyncServices` context manager, like so:

.. code-block:: python

    from compass.llm import LLMCallerArgs

    services = [...]  # list of service initializations here
    async with RunningAsyncServices(services):
        ...  # run your script here



Checking the Document for Ordinances
------------------------------------
The first thing we need to do to process out document is to verify that it contains ordinance information at all.
Even if you *know* your document contains ordinances, you will need to run this process since it stores the
text chunks from the document text that actually contain the ordinance info and allows us to run the next step -
ordinance text extraction. To run ordinance validation, we pass the document through the
:func:`~compass.extraction.apply.check_for_ordinance_info` function:

.. code-block:: python

    from compass.extraction.apply import check_for_ordinance_info
    from compass.extraction.solar import SolarHeuristic, SolarOrdinanceTextCollector

    doc = await check_for_ordinance_info(
        doc,
        llm_callers={"default": caller_args},
        heuristic=SolarHeuristic(),
        ordinance_text_collector_class=SolarOrdinanceTextCollector,
        permitted_use_text_collector_class=None,
    )


To call this function, we passed through the document and a dictionary specifying that the default LLM calling
args should be used for all processing (``{"default": caller_args}``). If we wanted to use different LLMs (e.g.
a different LLM for date extraction, which is part of the validation process, we would specify that in this
dictionary). We also specified that we want to use the ``SolarHeuristic``, which allows us to save on LLM costs
by applying a simple keyword search heuristic to each chunk before passing it to the LLM. Finally, we specified
that we want to use the ``SolarOrdinanceTextCollector`` class to check for solar ordinance text in the document
(as opposed to ``WindOrdinanceTextCollector``, for example, which would check for wind ordinance text).
We also left out the ``permitted_use_text_collector_class`` input, since we do not care to extract permitting
information for this example. If we did want to include it, we should pass
:class:`~compass.extraction.solar.SolarPermittedUseDistrictsTextCollector` as the value for the
``permitted_use_text_collector_class`` parameter.


Extracting the Ordinance Text
-----------------------------
The previous step flagged all the text chunks that contain ordinance information, but in practice those are still
too large and contain too much disparate information to be effective when used as context. Therefore, the next
step in the processing pipeline is to extract only the text that pertains to ordinances from these chunks. We
can do this using the :func:`~compass.extraction.apply.extract_ordinance_text_with_llm` function:

.. code-block:: python

    from compass.llm import LLMCaller
    from compass.extraction.apply import extract_ordinance_text_with_llm
    from compass.extraction.solar import SolarOrdinanceTextExtractor

    doc, ord_text_key = await extract_ordinance_text_with_llm(
        doc,
        caller_args.text_splitter,
        extractor=SolarOrdinanceTextExtractor(
            LLMCaller(llm_service=caller_args.llm_service)
        ),
        original_text_key="ordinance_text",
    )


The first argument to this function is our ordinance document, which should contain the ``"ordinance_text"``
key in it's ``doc.attrs`` dictionary. This key contains the (concatenated) text chunks known to contain
ordinances, and is automatically added for us by the
:func:`~from compass.extraction.apply.check_for_ordinance_info` function (assuming the text does indeed contain
ordinance text). Next, we pass the text splitter instance that will be used to split the concatenated chunks.
We also pass in a :class:`~compass.extraction.solar.SolarOrdinanceTextExtractor` instance which will be used
to actually extract the ordinance text. Finally, we tell the function that the concatenated text chunk text
is found under the ``"ordinance_text"`` key in ``doc.attrs``.

If successful, this function stores the extracted ordinance text in the ``doc.attrs`` dictionary under the
``ord_text_key`` key (the value of which depends on the text extractor instance we are using).


Extracting Ordinance Values
---------------------------
Finally we are ready to extract structured ordinance values from the ordinance text. We do this using the
:func:`~compass.extraction.apply.extract_ordinance_values` function:

.. code-block:: python

    from compass.extraction.apply import extract_ordinance_values
    from compass.extraction.solar import StructuredSolarOrdinanceParser

    doc = await extract_ordinance_values(
        doc,
        parser=StructuredSolarOrdinanceParser(
            llm_service=caller_args.llm_service
        ),
        text_key=ord_text_key,
        out_key="ordinance_values",
    )


The first argument to this function is our ordinance document, which should contain the ``ord_text_key``
key in it's ``doc.attrs`` dictionary (the value of this key depends on the text extractor instance we used
in `Extracting the Ordinance Text`_). This key contains ordinance text that we can use as context in the
decision trees that are executed to extract ordinance values. Next, we pass a
:class:`~compass.extraction.solar.StructuredSolarOrdinanceParser` instance, which is responsible for setting
up and running the decision trees and returning the results in structured format (typically CSV). Finally,
we tell the function which keys the input ordinance text and output structured values should go under in
the ``doc.attrs`` dictionary.


Putting it All Together
=======================
We can combine all these steps into one simple script to run from the command line. We'll add some logging
setup and calls to help us track the state of the processing on the terminal. Below is a sample script with
all of the aforementioned components:

.. literalinclude:: parse_pdf.py
    :language: python

You can also find the script `here <https://github.com/NREL/COMPASS/blob/main/examples/parse_existing_docs/parse_pdf.py>`_.


Executing the Script
====================
Now we are ready to run the full extraction script! In order for this to execute correctly, we need to set
environment variables describing our Azure OpenAI deployment:

.. code-block:: shell

    export AZURE_OPENAI_API_KEY=<your API key>
    export AZURE_OPENAI_ENDPOINT=<your API endpoint>
    export AZURE_OPENAI_VERSION=<your API version>

You may also need to change the model name in the script to match the model name of your deployment.
Once everything is set, we can run the script:

.. code-block:: shell

    python parse_pdf.py

You will see logging messages indicating the progress as the script executes. Don't worry if you see
``ERROR`` messages during the structured ordinance value extraction portion - this is normal and
expected (all they are telling you is that a decision tree branch reached a final leaf).

If everything executed correctly, you should see the following in your directory:

- ``Decatur County, Indiana Ordinance Text.txt``: This contains the subset of ordinance text
  from the document that contains ordinance information.
- ``Decatur County, Indiana Ordinances.csv``: This CSV file contains structured ordinance values
  extracted from the document.
