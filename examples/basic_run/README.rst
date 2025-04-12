*********************
Running INFRA-COMPASS
*********************

This example walks you through setting up and executing your first INFRA-COMPASS run.


Prerequisites
=============
We recommend enabling Optical Character Recognition (OCR) for PDF parsing, which
allows the program to process scanned documents. To enable OCR, you'll need to install ``pytesseract``.
If you installed COMPASS via PyPI, you may need to install a few additional dependencies:

.. code-block:: shell

    $ pip install pytesseract pdf2image

If you're using ``pixi`` to run the pipeline (recommended), these libraries are included by default.

In either case, you may still need to complete a few additional setup steps if this is your first time
installing Google's ``tesseract`` utility. Follow the installation instructions
`here <https://pypi.org/project/pytesseract/#:~:text=lang%5D%20image_file-,INSTALLATION,-Prerequisites%3A>`_.


Setting Up the Run Configuration
================================
The INFRA-COMPASS configuration file—written in either ``JSON`` or ``JSON5`` format—is a simple config that
defines parameters for running the process. Each key in the config corresponds to an argument for the function
`process_counties_with_openai <https://nrel.github.io/COMPASS/_autosummary/compass.scripts.process.process_counties_with_openai.html#compass.scripts.process.process_counties_with_openai>`_.
Refer to the linked documentation for detailed and up-to-date descriptions of each input.


Minimal Config
--------------
At a minimum, the INFRA-COMPASS config file requires three keys: ``"out_dir"``, ``"jurisdiction_fp"``, and ``"tech"``.

- ``"out_dir"``: Path to the output directory for results. This directory will be created if it doesn't already exist.
- ``"jurisdiction_fp"``: Path to a CSV file containing ``County`` and ``State`` columns. Each row represents a county to be processed. See the `example CSV <https://github.com/NREL/elm/blob/main/examples/basic_run/jurisdictions.csv>`_ for reference.
- ``"tech"``: A string specifying the technology or infrastructure focus of this run.

You can view a minimal working configuration in `config_bare_minimum.json5 <https://github.com/NREL/COMPASS/blob/main/examples/basic_run/config_bare_minimum.json5>`_.

.. literalinclude:: config_bare_minimum.json5
    :language: json5

As noted in the file comments, this setup assumes your LLM credentials and endpoints are configured via environment variables.
For example, if you're using Azure OpenAI, you should have the following set:

- ``AZURE_OPENAI_API_KEY``
- ``AZURE_OPENAI_VERSION``
- ``AZURE_OPENAI_ENDPOINT``

This minimal setup also assumes you're using the default LLM model configured for this tool - ``"gpt-4o"`` as of April 11, 2025.
To use a different model, simply add ``"model": "your-model-name"`` to your config (e.g., ``"gpt-4o-mini"``).


Typical Config
--------------
In most cases, you'll want more control over the execution parameters, especially those related to the LLM configuration.
You can review all available inputs in the
`process_counties_with_openai <https://nrel.github.io/COMPASS/_autosummary/compass.scripts.process.process_counties_with_openai.html#compass.scripts.process.process_counties_with_openai>`_
documentation. Our recommended configuration is shown in `config_recommended.json5 <https://github.com/NREL/COMPASS/blob/main/examples/basic_run/config_recommended.json5>`_.

.. literalinclude:: config_recommended.json5
    :language: json5

In this configuration, we define several LLM-related settings, including an increased token rate limit to speed up processing
and a larger text splitter chunk size (compared to the default) to include more context per query.

.. WARNING::

   Be cautious when adjusting the ``"text_splitter_chunk_size"``. Larger chunk sizes increase token usage, which may result in higher costs per query.

You can also specify LLM credentials and endpoint details directly in the config under the ``client_kwargs`` key.
Note that while this can be convenient for quick testing, storing credentials in plaintext is not recommended for production environments.

We also set ``"verify_ssl": false`` under ``"file_loader_kwargs"`` to avoid SSL certificate errors commonly encountered on the NREL VPN.
If you're not using the VPN, it's best to leave this value as the default (``true``).

Finally, as noted in the `Prerequisites`_ section, we recommend enabling OCR using ``pytesseract``. If you choose to do so, you must specify the
path to the ``tesseract`` executable using the ``pytesseract_exe_fp`` key. You can locate the path by running:

.. code-block:: shell

    $ which tesseract

To completely disable OCR, simply omit the ``pytesseract_exe_fp`` key from your config.


Kitchen Sink Config
-------------------

In `config_kitchen_sink.json5 <https://github.com/NREL/COMPASS/blob/main/examples/basic_run/config_recommended.json5>`_,
we show what a configuration might look like that utilizes all available parameters.

.. literalinclude:: config_kitchen_sink.json5
    :language: json5

We will not go into detail into all of the possible options here. Instead, we hope that the combination of this example
and the documentation will be enough to

.. NOTE:: Be sure to provide full paths to all files/directories unless you are executing the program from your working folder.


Execution
=========
Once you are happy with the configuration parameters, you can kick off the processing using

.. code-block:: shell

    $ compass process -c config.json

If you're using ``pixi``, activate the environment first:

.. code-block:: shell

    $ pixi shell
    $ compass process -c config.json5

or run with pixi directly:

.. code-block:: shell

    $ pixi run compass process -c config.json5

Replace ``config.json5`` with the path to your actual configuration file.

You may also wish to add a ``-v`` option to print logs to the terminal (however, keep in mind that the code runs
asynchronously, so the the logs will not print in order).

During execution, INFRA-COMPASS will:
1. Load and validate the jurisdiction CSV.
2. Attempt to locate and download relevant ordinance documents for each jurisdiction.
3. Parse and validate the documents.
4. Extract relevant ordinance text from the documents.
5. Parse the extracted text to determine the quantitative and qualitative ordinance values within, using decision tree-based LLM queries.
6. Output structured results to your configured ``out_dir``.

The runtime duration varies depending on the number of jurisdictions, the number of documents found for each jurisdiction,
and the rate limit/output token rate of the LLM(s) used.


Outputs
=======

After completion, you'll find several outputs in the ``out_dir``:

- **Extracted Ordinances**: Structured CSV files containing parsed ordinance values.
- **Ordinance Documents**: PDF or text (HTML) documents containing the legal ordinance.
- **Cleaned Text Files**: Text files containing the ordinance-specific text excerpts portions of the downloaded documents.
- **Metadata Files**: JSON files describing metadata parameters corresponding to your run.
- **Logs and Debug Files**: Helpful for reviewing LLM prompts and tracing any issues.

You can now use these outputs for downstream analysis, visualization, or integration with other NREL tools like
`reVX setbacks <https://nrel.github.io/reVX/misc/examples.setbacks.html>`_.
