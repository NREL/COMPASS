---
title: "Poppler runtime missing binaries"
date: "2025-08-28"
verified_on: "2025-08-28"
os: macos
arch: arm64
severity: medium
status: active
tags: [poppler, pdf2image, runtime]
reproducible: true
---

Symptom
-------

Runtime errors indicate Poppler utilities are missing, for example:

```
FileNotFoundError: [Errno 2] No such file or directory: 'pdftoppm'
```

Environment
-----------

- macOS host where Python calls `pdf2image` or other Poppler-dependent tooling

Reproduction steps
------------------

1. Run a PDF conversion that relies on Poppler (e.g., code that uses `pdf2image`).
2. Observe the FileNotFoundError or similar runtime error.

Resolution
----------

Install Poppler via Homebrew:

```bash
brew install poppler
```

Verify the binaries are on PATH:

```bash
which pdftoppm
which pdftotext
```

Notes
-----

`pdf2image` requires host-installed Poppler binaries. Pixi manages Python packages but not host-level system packages; install the latter with Homebrew, apt, or your OS package manager.

References
----------

- https://poppler.freedesktop.org/

Verified on
----------

- Apple M1 Pro (2021), macOS 15.6.1 (verification date: 2025-08-28)
