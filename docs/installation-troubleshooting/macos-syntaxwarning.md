---
title: "SyntaxWarning: invalid escape sequence in crawl4ai/prompts.py"
date: "2025-08-28"
verified_on: "2025-08-28"
os: macos
arch: arm64
severity: low
status: active
tags: [python, warning, syntax]
reproducible: true
---

Symptom
-------

You may see a warning like:

```
SyntaxWarning: invalid escape sequence '\`'
```

Environment
-----------

- A Python file in the repository (example: `crawl4ai/prompts.py`) contains a string with a backslash followed by a character that doesn't form a valid escape sequence.

Reproduction steps
------------------

1. Run code that imports or executes the affected module (or run linting/tests).
2. Observe the SyntaxWarning printed at import time or during execution.

Resolution
----------

Update the affected string to use a valid escape or a raw string. For example, either double-escape the backslash:

```py
# before
text = "This contains a backslash and a backtick: \`"

# after
text = "This contains a backslash and a backtick: \\\`"
```

or mark the string as raw (if that semantics is acceptable):

```py
text = r"This contains a backslash and a backtick: \`"
```

Notes
-----

This is a low-severity, linter-level issue; it won't usually break runtime behavior but fixing it keeps console output clean and avoids noisy CI logs.

References
----------
- Python string literal docs: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
