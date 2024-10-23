use pyo3::prelude::*;
use pyo3::types::PyString;
use serde::Serialize;

#[pyclass(eq, eq_int)]
#[derive(Copy, Debug, Clone, PartialEq, Serialize)]
pub enum Side {
    Long,
    Short,
}

#[pyclass(eq, eq_int)]
#[derive(Copy, Debug, Clone, PartialEq, Serialize)]
pub enum OrderType {
    Market,
    Limit,
}

impl ToPyObject for Side {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Side::Long => PyString::new_bound(py, "Long").into_py(py),
            Side::Short => PyString::new_bound(py, "Short").into_py(py),
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(Copy, Debug, Clone, PartialEq, Serialize)]
pub enum CloseReason {
    TakeProfit,
    StopLoss,
    Signal,
}
impl ToPyObject for CloseReason {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            CloseReason::TakeProfit => PyString::new_bound(py, "TakeProfit").into_py(py),
            CloseReason::StopLoss => PyString::new_bound(py, "StopLoss").into_py(py),
            CloseReason::Signal => PyString::new_bound(py, "Signal").into_py(py),
        }
    }
}
