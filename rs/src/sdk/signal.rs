use pyo3::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone, Serialize)]
pub struct Signal {
    pub index: Decimal,
    pub side: Decimal,
    pub signal_type: String,
    pub order_type: String,
}

pub type Signals = HashMap<Decimal, Vec<Signal>>;
