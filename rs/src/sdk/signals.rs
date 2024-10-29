use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Signals {
    pub long_signals: Vec<bool>,
    pub short_signals: Vec<bool>,
}

#[pymethods]
impl Signals {
    #[new]
    fn new(long_signals: Vec<bool>, short_signals: Vec<bool>) -> Self {
        Signals {
            long_signals,
            short_signals,
        }
    }
}
