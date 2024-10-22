use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass(get_all)]
#[derive(Debug)]
pub struct SharedState {
    pub equity: Py<PyList>,
}
#[pymethods]
impl SharedState {}
