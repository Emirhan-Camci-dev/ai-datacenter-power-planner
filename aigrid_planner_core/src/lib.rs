pub mod power_flow;
pub mod thermal;
pub mod models;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn aigrid_planner_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<models::FacilityState>()?;
    Ok(())
}
