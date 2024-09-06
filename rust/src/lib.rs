use numpy::ndarray::{ArrayD, ArrayViewD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::*;

/// Basic example. Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string_rust(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Multiplies two numpy arrays.
/// Uses immutable borrows producing a new array
fn mult_arrays(x: ArrayViewD<'_, f64>, y: ArrayViewD<'_, f64>) -> ArrayD<f64> {
    &x * &y
}

/// Wrapper function around mult_arrays to deal with Python details.
/// Note that f64 types are used throughout - python side must comply or
/// you'll get a runtime TypeError.
#[pyfunction]
fn mult_arrays_rust<'py>(
    py: Python<'py>,
    x: PyReadonlyArrayDyn<'py, f64>,
    y: PyReadonlyArrayDyn<'py, f64>,
) -> Bound<'py, PyArrayDyn<f64>> {
    let x = x.as_array();
    let y = y.as_array();
    let z = mult_arrays(x, y);
    z.into_pyarray_bound(py)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string_rust, m)?)?;
    m.add_function(wrap_pyfunction!(mult_arrays_rust, m)?)?;
    Ok(())
}

/// Rust Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_as_string() {
        assert_eq!(sum_as_string_rust(1, 2).unwrap(), "3");
    }
}
