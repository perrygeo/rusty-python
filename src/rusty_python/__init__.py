"""
Rusty Python.
An example python module using native python extensions.
"""

# Import select functions/classes from the native submodule
# AND from the python core submodule into a single public API.
# Functions defined in `lib.rs` and `lib.py` are both available.

from .rusty_python import sum_as_string, mult_arrays
from .lib import sum_as_string_py

__all__ = [
    mult_arrays,
    sum_as_string_py,
    sum_as_string,
]
