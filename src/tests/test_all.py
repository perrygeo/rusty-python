import pytest

# Basic example in rust and python
from rusty_python import sum_as_string_rust
from rusty_python import sum_as_string

# Numpy interop
import numpy as np
from rusty_python import mult_arrays_rust


def test_sum_as_string_rust():
    assert sum_as_string_rust(1, 1) == "2"


def test_sum_as_string():
    assert sum_as_string(1, 1) == "2"


def test_numpy_mult():
    a = np.array([1, 2, 3, 4], dtype=np.float64)  # matches f64 in Rust
    b = np.array([5, 6, 7, 8], dtype=np.float64)
    c = mult_arrays_rust(a, b)
    assert c.sum() == 70


def test_numpy_mult_shape():
    # shape doesn't matter
    a = np.array([1, 2, 3, 4], dtype=np.float64)  # matches f64 in Rust
    b = np.array([5, 6, 7, 8], dtype=np.float64)
    d = mult_arrays_rust(a.reshape(2, 2), b.reshape(2, 2))
    assert d.sum() == 70


def test_numpy_mult_dtype():
    # But dtype is strict
    x = np.array([1, 2, 3, 4], dtype=np.int32)  # not f64!
    y = np.array([5, 6, 7, 8], dtype=np.int32)
    with pytest.raises(TypeError):
        mult_arrays_rust(x, y)
