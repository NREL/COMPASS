---
title: "Pixi warning: conda-packages overridden by PyPI"
date: "2025-08-28"
verified_on: "2025-08-28"
os: macos
arch: any
severity: low
status: active
tags: [pixi, conda, pip, environment]
reproducible: true
---

Symptom
-------

When running `pixi shell -e pdev`, you may see a warning such as:

```
WARN These conda-packages will be overridden by pypi:
   yarg
```

Environment
-----------

- Using Pixi to create or enter a project environment that mixes Conda and PyPI package sources.

Reproduction steps
------------------

1. Run `pixi shell -e pdev` (or the equivalent Pixi environment creation command).
2. Observe the warning during environment resolution.

Resolution
----------

This warning indicates that a package available from both Conda and PyPI will be installed from PyPI instead of the Conda channel. Usually this is safe, but if you rely on the Conda-built binary (for example, for performance or compatibility), align the package source by:

- Pinning the package to the Conda channel in the Pixi/Pixienv configuration.
- Removing the PyPI override from the project's requirements if Conda should be authoritative.

Notes
-----

Most users can safely ignore this warning unless they encounter runtime issues tied to a specific build of the package.
