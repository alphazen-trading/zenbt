use pyo3::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::enums::Side;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Signal {
    pub index: Decimal,
    pub side: Side,
    pub signal_type: String,
    pub order_type: String,
}

pub type Signals = HashMap<Decimal, Vec<Signal>>;
