use numpy::ndarray::{ArrayD, ArrayViewD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::prelude::*;

/// Basic example. Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Multiplies two numpy arrays.
/// Uses immutable borrows producing a new array
fn mult_array_views(x: ArrayViewD<'_, f64>, y: ArrayViewD<'_, f64>) -> ArrayD<f64> {
    &x * &y
}

/// Wrapper function around mult_arrays to deal with Python details.
/// Note that f64 types are used throughout - python side must comply or
/// you'll get a runtime TypeError.
#[pyfunction]
fn mult_arrays<'py>(
    py: Python<'py>,
    x: PyReadonlyArrayDyn<'py, f64>,
    y: PyReadonlyArrayDyn<'py, f64>,
) -> Bound<'py, PyArrayDyn<f64>> {
    let x = x.as_array();
    let y = y.as_array();
    let z = mult_array_views(x, y);
    z.into_pyarray_bound(py)
}

/// A Python module implemented in Rust.
/// The fn name must match Cargo.toml and is what you use to import in Python
#[pymodule]
fn rusty_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(mult_arrays, m)?)?;
    Ok(())
}

/// Rust Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_as_string() {
        assert_eq!(sum_as_string(1, 2).unwrap(), "3");
    }
}
