# Rusty Python

A brief introduction to mixing Rust code into a Python project.

## Motivation

Python has long been a dynamic extension language for native C/C++ code (using techniques such as
SWIG, Cython, pybind, etc or writing the Python C API directly).
Providing ergonomic wrappers to native code is a foundational part of Python's success and 
it was arguably *the* fundamental purpose of the language from the beginnning.
(See David Beazley's [2021 Keynote](https://www.youtube.com/watch?v=riuyDEHxeEo)).

But C/C++ code has its problems, mainly with memory safety. It's up to the programmer to make sure
all memory access is safe, and that responsibility has empirically proven too complex for 
software developers. This results in bugs, crashes and security vulnerabilities.
The US Government has even acknowledged these problems publicly,
stating that "technology manufacturers can prevent entire classes of vulnerabilities from entering the digital 
ecosystem by adopting memory safe programming languages." ([Source](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/)).

Rust attempts to solve these problems not with a garbage-collected runtime but with a novel
borrow checker that provides compile-time safety. 
If C speed plus memory safety sounds appealing, Rust might be for you.

This repo explores writing Rust extensions *within* a Python project source tree. We use `PyO3` to 
generate bindings and `maturin` as a build tool to smooth the process.


## Development

```
$ pip install maturin
```

Every time the rust source code changes:

```
$ maturin develop
```

Edit `rust/src/lib.rs` to change the rust code 
and `src/rusty-python/core.py` to change the python side. Both modules are
loaded into the top-level module so you can access them like so:

```python
>>> from rusty_python import sum_as_string, sum_as_string_rust
>>> sum_as_string(9000, 1)
'9001'
>>> sum_as_string_rust(9000, 1)
'9001'
```

To run the test suite

```bash
$ python -m pytest -v
==================== test session starts =====================
platform linux -- Python 3.12.4, pytest-8.3.2, pluggy-1.5.0 -- /home/mperry/.virtualenvs/pyo3/bin/python
cachedir: .pytest_cache
rootdir: /home/mperry/projects/rusty-python
configfile: pyproject.toml
collected 2 items

src/tests/test_all.py::test_sum_as_string_rust PASSED  [ 50%]
src/tests/test_all.py::test_sum_as_string PASSED       [100%]

===================== 2 passed in 0.02s ======================
```

## When to go native

The rule of thumb is to build your code in Python as normal, profile
your code for hot spots (code regions that could benefit from native code)
and re-implement those functions in Rust.


## Starting from scratch

This repo was created using `maturin new`. To create from scratch, in a new virtualenv

```
$ maturin new --mixed --src rusty-python
```

Which gives you a directory structure like

```
rusty-python
├── pyproject.toml
├── README.md
├── rust
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── src
    ├── rusty_python
    │   └── __init__.py
    └── tests
        └── test_all.py
```

* `src/` holds your python module and tests, same as the standard python module layout.
* `rust/` contains any rust extentions to add to your python module.
