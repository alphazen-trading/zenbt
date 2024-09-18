use pyo3::prelude::*;
use pyo3::types::PyString;

#[derive(Copy, Debug, Clone, PartialEq)]
#[pyclass]
pub enum Side {
    Long,
    Short,
}

impl ToPyObject for Side {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Side::Long => PyString::new(py, "Long").into_py(py),
            Side::Short => PyString::new(py, "Short").into_py(py),
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
#[pyclass]
pub enum CloseReason {
    TakeProfit,
    StopLoss,
}
impl ToPyObject for CloseReason {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            CloseReason::TakeProfit => PyString::new(py, "TakeProfit").into_py(py),
            CloseReason::StopLoss => PyString::new(py, "StopLoss").into_py(py),
        }
    }
}
