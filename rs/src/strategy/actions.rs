use pyo3::prelude::*;

#[pyclass(get_all, frozen)]
#[derive(Debug, Clone)]
pub struct Action {
    pub test: f64,
}

#[pymethods]
impl Action {
    #[new]
    fn new(test: f64) -> PyResult<Action> {
        Ok(Action { test })
    }
}
