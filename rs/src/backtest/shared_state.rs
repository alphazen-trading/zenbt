use pyo3::prelude::*;
use pyo3::types::PyList;

use crate::sdk::position::Positions;

#[pyclass(get_all)]
#[derive(Debug)]
pub struct SharedState {
    pub equity: Py<PyList>,
    pub active_positions: Py<PyList>,
    pub closed_positions: Py<PyList>,
}
#[pymethods]
impl SharedState {}
