---
title: "Tesseract not found at runtime"
date: "2025-08-28"
verified_on: "2025-08-28"
os: macos
arch: arm64
severity: medium
status: active
tags: [tesseract, pytesseract, ocr]
reproducible: true
---

Symptom
-------

OCR calls raise errors indicating the `tesseract` executable is missing, e.g.:

```
FileNotFoundError: [Errno 2] No such file or directory: 'tesseract'
```

Environment
-----------

- macOS host with `pytesseract` installed in the Pixi-managed Python environment

Verified on
----------

- Apple M1 Pro (2021), macOS 15.6.1 (verification date: 2025-08-28)

Reproduction steps
------------------

1. Run an OCR task using COMPASS or a small script that imports `pytesseract`.
2. Observe the FileNotFoundError or similar runtime error.

Resolution
----------

Install Tesseract via Homebrew:

```bash
brew install tesseract
```

Confirm the executable is available on PATH:

```bash
which tesseract
```

If the runtime still cannot find `tesseract`, explicitly set the path for `pytesseract` in code (example):

```py
import pytesseract
pytesseract.pytesseract.tesseract_cmd = "/opt/homebrew/bin/tesseract"
```

Notes
-----

`pytesseract` is a Python wrapper that invokes the system `tesseract` binary; the binary is provided by the OS package manager, not by Python packages.

References
----------

- https://pypi.org/project/pytesseract/
