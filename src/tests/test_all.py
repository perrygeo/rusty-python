from rusty_python import sum_as_string_rust
from rusty_python import sum_as_string


def test_sum_as_string_rust():
    assert sum_as_string_rust(1, 1) == "2"


def test_sum_as_string():
    assert sum_as_string(1, 1) == "2"
