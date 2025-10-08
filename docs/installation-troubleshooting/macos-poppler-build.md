---
title: "Poppler build failure: missing macOS SDK headers"
date: "2025-08-28"
verified_on: "2025-08-28"
os: macos
arch: arm64
severity: medium
status: active
tags: [poppler, build, xcode]
reproducible: true
---

Symptom
-------

Compilation fails with errors like:

```
fatal error: 'ctime' file not found
```

or other C/C++ header errors when installing packages that build native extensions (for example, Poppler utilities or bindings).

Environment
-----------

- macOS host (any recent release)
- Building Python packages that include C/C++ extensions

Reproduction steps
------------------

1. Create or update the Pixi environment that installs packages requiring native builds (or run `pip install` for such packages).
2. Observe the build failure during the compilation step.

Resolution
----------

1. Install or update the Xcode Command Line Tools:

```bash
xcode-select --install
```

2. If the system points to the wrong developer directory, reset it:

```bash
sudo xcode-select --reset
```

3. If prompted, accept the Xcode license:

```bash
sudo xcodebuild -license
```

4. If the build still cannot find headers, export a valid SDK path for the build process and retry:

```bash
export SDKROOT=$(xcrun --sdk macosx --show-sdk-path)
# then re-run the install or build
```

Notes
-----

These steps provide the compiler and headers required to build C/C++ extensions. Prefer installing Poppler via the OS package manager (Homebrew) when possible to avoid local compilation.

References
----------

- https://developer.apple.com

Verified on
----------

- Apple M1 Pro (2021), macOS 15.6.1 (verification date: 2025-08-28)
