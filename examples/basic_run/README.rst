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
The INFRA-COMPASS configuration file (JSON or JSON5 format) is a simple config that describes the parameters of
the process execution. All of the keys in this configuration file should be arguments to the
`process_counties_with_openai <https://nrel.github.io/COMPASS/_autosummary/compass.scripts.process.process_counties_with_openai.html#compass.scripts.process.process_counties_with_openai>`_
function. Please refer to the documentation for more detailed and up-to-date explanations of each input.


Minimal Setup
-------------
The simplest INFRA-COMPASS configuration requires just three keys: ``"out_dir"``, ``"jurisdiction_fp"``, and ``"tech"``.
The ``"out_dir"`` key should point to a directory where the outputs of the scraping and parsing routines should
go. This directory does not have it exist; it will be created if missing. The ``"jurisdiction_fp"`` should
point to a CSV file with ``County`` and ``State`` columns. Each row of this CSV file should be populated with a
single county to be processed. Finally, the ``"tech"`` key should be a string representing the technology or
infrastructure that you are running INFRA-COMPASS for.

.. include:: config_bare_minimum.json5
    code: json
