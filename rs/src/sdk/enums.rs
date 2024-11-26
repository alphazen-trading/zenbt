use pyo3::prelude::*;
use pyo3::types::PyString;
use serde::Serialize;

#[pyclass(eq, eq_int)]
#[derive(Copy, Debug, Clone, PartialEq, Serialize)]
/// An enum representing the side of the order or position
///
/// Attributes:
///     Long: Sets the side of a Position or Order to Long
///     Short: Sets the side of a Position or Order to Short
pub enum Side {
    Long,
    Short,
}

#[pyclass(eq, eq_int)]
#[derive(Copy, Debug, Clone, PartialEq, Serialize)]
/// An enum representing the order type
pub enum OrderType {
    /// Sets the type of a Position or Order to a Market order
    Market,
    /// Sets the type of a Position or Order to a Limit order
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
/// An enum representing the reason why an active position was closed
pub enum CloseReason {
    TakeProfit,
    /// Position was closed because of stop loss
    StopLoss,
    /// Position was closed because an opposite signal was triggered
    Signal,
    /// Position was closed manually
    Manual,
}
impl ToPyObject for CloseReason {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            CloseReason::TakeProfit => PyString::new_bound(py, "TakeProfit").into_py(py),
            CloseReason::StopLoss => PyString::new_bound(py, "StopLoss").into_py(py),
            CloseReason::Signal => PyString::new_bound(py, "Signal").into_py(py),
            CloseReason::Manual => PyString::new_bound(py, "Manual").into_py(py),
        }
    }
}
