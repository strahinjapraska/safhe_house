use pyo3::prelude::*; 
use crate::math::discrete_gaussian::sample_z;



#[pyfunction]
fn sample_discrete_gaussian(s: f64, n: usize) -> i32{
    sample_z(s, n)
}

#[pymodule]
fn safhe_house(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sample_discrete_gaussian, m)?)?;
    Ok(())
}