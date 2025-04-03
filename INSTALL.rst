Installing INFRA-COMPASS
========================

.. inclusion-install

Installing from PyPI
--------------------

INFRA-COMPASS can be installed via pip from
`PyPI <https://pypi.org/project/NREL-COMPASS>`__.

.. code-block:: shell

    pip install nrel-compass

.. note::
    You must have ``pip>=19.3`` to install from PyPI.

.. note::

    It is recommended to install and run INFRA-COMPASS from a virtual environment, for example,
    using `Miniconda <https://www.anaconda.com/docs/getting-started/miniconda/main>`__.


Handling ImportErrors
---------------------

If you encounter an ``ImportError``, it usually means that Python couldn't find INFRA-COMPASS in the list of available
libraries. Python internally has a list of directories it searches through, to find packages. You can
obtain these directories with.

.. code-block:: python

    import sys
    sys.path

One way you could be encountering this error is if you have multiple Python installations on your system
and you don't have INFRA-COMPASS installed in the Python installation you're currently using.
In Linux/Mac you can run ``which python`` on your terminal and it will tell you which Python installation you're
using. If it's something like "/usr/bin/python", you're using the Python from the system, which is not recommended.
Instead, it is highly recommended to use an isolated environment, such as one created using
`Miniconda <https://www.anaconda.com/docs/getting-started/miniconda/main>`__, for package and dependency updates.


Installing from source
----------------------

The installation instruction below assume that you have python installed
on your machine and are using `conda <https://docs.conda.io/en/latest/index.html>`_
or `miniconda <https://www.anaconda.com/docs/getting-started/miniconda/main>`__ as your
package/environment manager.

1. Clone the `COMPASS` repository.
    - Using ssh: :code:`git clone git@github.com:NREL/COMPASS.git`
    - Using https: :code:`git clone https://github.com/NREL/COMPASS.git`

2. Create and activate  the ``compass`` environment and install the package:
    1) Create a conda env: ``conda create -n compass python=3.12``
    2) Activate the newly-created conda env: ``conda activate compass``
    3) Change directories into the repository: ``cd compass``
    4) Prior to running ``pip`` below, make sure the branch is correct (install from main!): ``git branch -vv``
    5) Install ``compass`` and its dependencies by running:
       ``pip install -e .`` (or ``pip install -e .[dev]`` if running a dev branch or working on the source code)


Installing the development version of INFRA-COMPASS
---------------------------------------------------

We use `pixi <https://pixi.sh/latest/>`_ to manage environments across developers.
This tool allows developers to install libraries and applications in a reproducible
way across multiple platforms. This means bugs are easier to reproduce, and it's easier
to move your development environment to a new piece of hardware.

We keep a version-controlled ``pixi.lock`` in the repository to allow locking with
the full requirements tree so we can reproduce behaviors and bugs and easily compare
results.

You can use the ``pdev`` feature in ``pixi`` to get all necessary python development tools
(after cloning the repository)::

    $ pixi shell -e pdev

To work on the Rust-based CLI, you can use the ``rdev`` feature instead::

    $ pixi shell -e rdev

You are welcome to use a different environment manager (e.g. ``conda``, ``mamba``, etc),
but we make no promises to provide support on environment-related issues/bugs in this case.
