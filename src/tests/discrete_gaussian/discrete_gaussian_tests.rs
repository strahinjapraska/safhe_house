use pyo3::prelude::*; 
use crate::math::discrete_gaussian::sample_z;



#[pyfunction]
fn sample_discrete_gaussian() -> u64{
    sample_z(2.3, 64)
}

#[pymodule]
fn sample_discrete_gaussian_binding(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sample_discrete_gaussian, m)?)?;
    Ok(())
}