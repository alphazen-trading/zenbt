use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::enums::Side;

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
pub struct Signal {
    pub index: Decimal,
    pub side: Side,
    pub signal_type: String,
    pub order_type: String,
}

// pub type Signals = HashMap<Decimal, Vec<Signal>>;

// pub struct Signals {
//     pub long_entries: PyArrayDyn<bool>,
//     pub long_exits: PyArrayDyn<bool>,
//     pub short_entries: PyArrayDyn<bool>,
//     pub short_exits: PyArrayDyn<bool>,
// }
