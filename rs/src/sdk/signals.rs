use pyo3::prelude::*;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Signals {
    pub long_signals: Vec<bool>,
    pub short_signals: Vec<bool>,
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Signals {
    #[new]
    fn new(
        long_signals: Vec<bool>,
        short_signals: Vec<bool>,
    ) -> Self {
        Signals {
            long_signals,
            short_signals,
        }
    }
    #[getter]
    fn print(&self) {
        println!("{:?}", self);
    }
}
