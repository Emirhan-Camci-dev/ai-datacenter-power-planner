use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityState {
    pub total_it_load_kw: f64,
    pub cooling_load_kw: f64,
    pub pue: f64,
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyFacilityState {
    pub inner: FacilityState,
}

impl FacilityState {
    pub fn new(total_it_load_kw: f64, cooling_load_kw: f64) -> Self {
        let pue = (total_it_load_kw + cooling_load_kw) / total_it_load_kw;
        Self {
            total_it_load_kw,
            cooling_load_kw,
            pue,
        }
    }
}
