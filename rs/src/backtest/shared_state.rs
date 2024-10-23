use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pyclass(get_all)]
#[derive(Debug)]
pub struct SharedState {
    pub equity: Py<PyList>,
    pub active_positions: Py<PyDict>,
    pub closed_positions: Py<PyDict>,
}
#[pymethods]
impl SharedState {}
