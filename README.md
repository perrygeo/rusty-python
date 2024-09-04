# Rusty Python

[![CI](https://github.com/perrygeo/rusty-python/actions/workflows/CI.yml/badge.svg)](https://github.com/perrygeo/rusty-python/actions/workflows/CI.yml)

A brief introduction to mixing Rust code into a Python project.

## Motivation

Python has long been a dynamic extension language for native C/C++ code, using techniques such as
SWIG, Cython, pybind, etc. or writing the Python C API directly.

Providing ergonomic wrappers to native code is a foundational part of Python's success and 
it was arguably *the* purpose of the language from the beginning;
See David Beazley's [PyData 2021 Keynote](https://www.youtube.com/watch?v=riuyDEHxeEo).

But C/C++ code has its problems, mainly with memory safety. It's up to the programmer to make sure
all memory access is safe, and that responsibility has empirically proven too complex for 
software developers. This results in bugs, crashes and security vulnerabilities.
The US Government has even acknowledged these problems publicly,
stating that "technology manufacturers can prevent entire classes of vulnerabilities from entering the digital 
ecosystem by adopting memory safe programming languages." ([source](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/)).

Rust attempts to solve these problems not with a garbage-collected runtime,
but with a novel borrow checker that enforces safety . 
If C speed plus compile-time memory safety sounds appealing, Rust might be for you.

This repo explores writing Rust extensions *within* a Python project source tree. We use `PyO3` to 
generate bindings and `maturin` as a build tool to smooth the process.


## Development

In a new virtualenv

```
$ pip install maturin pytest
```

Every time the rust source code changes, use `make test` to rebuild, run the rust tests, and run the python tests.

```
$ make test
maturin develop
📦 Including license file "/home/mperry/projects/rusty-python/LICENSE"
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at /home/mperry/.virtualenvs/rusty-python/bin/python
📡 Using build options features from pyproject.toml
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
📦 Built wheel for CPython 3.12 to /tmp/.tmpeOscoW/rusty_python-0.1.0-cp312-cp312-linux_x86_64.whl
✏️   Setting installed package as editable
🛠 Installed rusty-python-0.1.0
cd rust && cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/rusty_python-89f82543e284a4d7)

running 1 test
test tests::test_sum_as_string ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

python -m pytest -v
================================================== test session starts ===================================================
platform linux -- Python 3.12.5, pytest-8.3.2, pluggy-1.5.0 -- /home/mperry/.virtualenvs/rusty-python/bin/python
cachedir: .pytest_cache
rootdir: /home/mperry/projects/rusty-python
configfile: pyproject.toml
collected 2 items                                                                                                 

src/tests/test_all.py::test_sum_as_string_rust PASSED                                                              [ 50%]
src/tests/test_all.py::test_sum_as_string PASSED                                                                   [100%]

=================================================== 2 passed in 0.03s ====================================================
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

To build a wheel for release, use `make build`

```bash
$ make build
maturin build --release
📦 Including license file "/home/mperry/projects/rusty-python/LICENSE"
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings
🐍 Found CPython 3.12 at /home/mperry/.virtualenvs/rusty-python/bin/python
📡 Using build options features from pyproject.toml
    Finished `release` profile [optimized] target(s) in 0.01s
📦 Built wheel for CPython 3.12 to /home/mperry/projects/rusty-python/rust/target/wheels/rusty_python-0.1.0-cp312-cp312-manylinux_2_34_x86_64.whl
```


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
