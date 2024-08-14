"""
Import everything from the native submodule AND from the python core submodule into
a single namespace. This allows functions defined in lib.rs and core.py to both
be available as a top-level module imports.
"""

from .rusty_python import *
from .core import *


__doc__ = rusty_python.__doc__

if hasattr(rusty_python, "__all__"):
    __all__ = rusty_python.__all__
