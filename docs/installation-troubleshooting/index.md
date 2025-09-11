# Installation Troubleshooting

This directory contains concise, platform-specific troubleshooting notes focused on reproducible diagnostics and resolutions.

Files:

- `macos.md` — macOS troubleshooting index and links to per-entry files.
- `macos-poppler-build.md` — Poppler build failure entry.
- `macos-poppler-runtime.md` — Poppler runtime binaries entry.
- `macos-tesseract.md` — Tesseract / pytesseract entry.
- `linux.md` — placeholder for Linux troubleshooting entries.
- `windows.md` — placeholder for Windows troubleshooting entries.

Contribution guidance

When adding a troubleshooting entry, include the following fields to make the entry reproducible and searchable:

- Title: short, descriptive title.
- Symptom: exact error message or observable behavior.
- Environment: OS, architecture (x86_64 / arm64), Python version, Pixi version (if relevant), and any other relevant software versions.
- Reproduction steps: exact commands or minimal steps to reproduce the issue.
- Resolution: exact commands and a brief explanation of why the resolution works.
- References: links to upstream docs, issues, or relevant resources.


Format example (minimal template with YAML frontmatter for new troubleshooting entries):

```yaml
---
title: "Brief title"
date: "YYYY-MM-DD"       # Use YYYY-MM-DD (e.g., "2025-08-28")
verified_on: null        # Set to validation date (e.g., "2025-08-29") after verification
os: macos                # single value or a list: ["macos", "linux", "windows"]
arch: any                # one of: "x86_64", "arm64", "any"
severity: low            # one of: "low", "medium", "high"
status: active           # "active" or "deprecated"
tags: [technology, error-type]  # e.g., [poppler, pdf2image]
reproducible: true
---
```

Symptom: copy-paste of error/output
Environment: e.g. macOS 13.5, arm64, Python 3.12, pixi 0.1
Reproduction steps:
  1. command1
  2. command2
Resolution:
  - command(s)
  - short explanation
References: link(s)

Notes:

- Keep entries concise and diagnostic-first; avoid duplicating general installation instructions already present in `README.rst` or Pixi docs.
- Per-OS files for clarity and searchability. If an entry is cross-platform, note that in the Environment field and add it to each relevant file.

Maintenance guidance:

- Each entry should include `date` and, when possible, `verified_on` to indicate when it was validated. Maintainers should mark `status: deprecated` if an entry becomes obsolete and point to the canonical install docs.
- Automated checks should verify frontmatter presence and basic fields.

## Docs tooling

The repository provides a small local tool to validate troubleshooting entries and generate a consolidated index. This tool is optional and non-blocking by default.

Run the validator and index generator inside the project's Pixi environment:

```bash
pixi run python3 tools/generate_troubleshooting_index.py
```

Use `--strict` to make the tool exit non-zero when required frontmatter is missing (useful for CI once the team decides to enforce metadata):

```bash
pixi run python3 tools/generate_troubleshooting_index.py --strict
```

Minimal frontmatter template for an entry (keep entries short and link to canonical README installation instructions):

```yaml
---
title: "Short descriptive title"
date: 2025-08-28
os: macos
arch: any
status: active
tags: [poppler, pdf2image]
---
```

The tool writes `docs/installation-troubleshooting/_generated_index.md`. You do not need to commit the generated file; it can be regenerated on demand. For now, the script will only warn about missing metadata unless `--strict` is used.

