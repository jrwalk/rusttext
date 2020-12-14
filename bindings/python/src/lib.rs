use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn foo() -> PyResult<String> {
    return Ok(String::from("foo"));
}

#[pymodule]
fn rusttext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(foo, m)?)?;

    Ok(())
}
