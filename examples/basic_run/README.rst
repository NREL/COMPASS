*********************
Running INFRA-COMPASS
*********************

This example walks you through the setup of your first INFRA-COMPASS execution.


Prerequisites
=============
We recommend executing the pipeline with Optical Character Recognition (OCR) enabled for PDF parsing, which
allows the program to parse scanned documents. To enable this option, you will need to install ``pytesseract``.
If you installed COMPASS from PyPi, you may need to install a few additional dependencies:

.. code-block:: shell

    $ pip install pytesseract pdf2image

If you plan to execute the pipeline with ``pixi`` (recommended), then you will have access to these libraries
by default. In either case, you may need to follow a few additional installation steps if this is your first
time using Google's ``tesseract`` utility on your machine. Please follow the installation instructions
`here <https://pypi.org/project/pytesseract/#:~:text=lang%5D%20image_file-,INSTALLATION,-Prerequisites%3A>`_.


Setting up the Run Configuration
================================
The INFRA-COMPASS configuration file (``JSON`` or ``JSON5`` format) is a simple config that describes the
parameters of the process execution. All of the keys in this configuration file should be arguments to the
`process_counties_with_openai <https://nrel.github.io/COMPASS/_autosummary/compass.scripts.process.process_counties_with_openai.html#compass.scripts.process.process_counties_with_openai>`_
function. Please refer to the documentation for more detailed and up-to-date explanations of each input.


Minimal Setup
-------------
The simplest INFRA-COMPASS configuration requires just three keys: ``"out_dir"``, ``"jurisdiction_fp"``, and ``"tech"``.
The ``"out_dir"`` key should point to a directory where the outputs of the scraping and parsing routines should
go. This directory does not have it exist; it will be created if missing. The ``"jurisdiction_fp"`` should
point to a CSV file with ``County`` and ``State`` columns. Each row of this CSV file should be populated with a
single county to be processed. See the
`example CSV <https://github.com/NREL/elm/blob/main/examples/basic_run/jurisdictions.csv>`_
file for reference. Finally, the ``"tech"`` key should be a string representing the technology or
infrastructure that you are running INFRA-COMPASS for. A basic example config is given in
`config_bare_minimum.json5 <https://github.com/NREL/COMPASS/blob/main/examples/basic_run/config_bare_minimum.json5>`_.

.. include:: config_bare_minimum.json5
    :code:

As mentioned in the comment in the config file, this does assume that your LLM endpoints and API keys have
been configured using environment variables (i.e. if you are using Azure OpenAI, then you have set
``AZURE_OPENAI_API_KEY``, ``AZURE_OPENAI_VERSION``, and ``AZURE_OPENAI_ENDPOINT`` to point to your OpenAI
deployment). This also assumes that you have deployed and would like to use the default LLM that we have
used for testing this tool, which is ``"gpt-4o"`` as of April 11, 2025. If you'd like to use a different model,
simply add ``"model": "gpt-4o-mini"`` to your config, replacing the value with the name of the model you would
like to use.


Typical Setup
-------------
In most cases, you'll want more control over some of the parameters of the execution (often pertaining to the
LLM configuration). You can always refer to the parameters of the
to the
`process_counties_with_openai <https://nrel.github.io/COMPASS/_autosummary/compass.scripts.process.process_counties_with_openai.html#compass.scripts.process.process_counties_with_openai>`_
for all available inputs, but our recommended configuration is shown in
`config_recommended.json5 <https://github.com/NREL/COMPASS/blob/main/examples/basic_run/config_recommended.json5>`_.


.. include:: config_recommended.json5
    :code:


In this configuration, we specified quite a few more specifics about the LLM we would like to use, notably setting
the rate limit (in tokens/min) to be quite large in order to process the documents as quickly as possible. We also
increase the text splitter chunk size (compared to the default) in order to fit more context per query.

.. WARNING:: Be careful when setting the setting the ``"text_splitter_chunk_size"``. A larger chunk size will
             cost more per query, since a lot more tokens are being used for the context.

By specifying the model details this way, we can also provide the LLM endpoint and API key information directly in
the config file via the ``client_kwargs`` input. Note that it typically not a good idea to keep credentials like
this in an unencrypted plaintext file; however, this can be useful for quick-and-dirty test runs.

We also specified ``"verify_ssl": false`` under ``"file_loader_kwargs"`` to avoid some certificate errors when running
on the NREL VPN. If you are running outside of NREL, you should consider leaving this argument set as the default
(``true``).

Finally, as mentioned in the `Prerequisites`_ section, we recommend setting up OCR via ``pytesseract``. If you have
chosen to do so, you should specify the path to the ``tesseract`` executable via the ``pytesseract_exe_fp`` key.
You can find the path to the ``tesseract`` executable by running the following command (after installing):

.. code-block:: shell

    $ which tesseract



Execution
=========
Once you are happy with the configuration parameters, you can kick off the processing using

.. code-block:: bash

    $ compass process -c config.json

You may also wish to add a ``-v`` option to print logs to the terminal (however, keep in mind that the code runs
asynchronously, so the the logs will not print in order).
