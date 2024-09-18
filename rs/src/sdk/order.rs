use pyo3::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::enums::Side;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Order {
    pub index: Decimal,
    pub price: Decimal,
    pub size: Decimal,
    pub sl: Decimal,
    pub tp: Decimal,
    pub side: Side,
    pub order_type: String,
}

pub type LimitOrders = HashMap<Decimal, Vec<Order>>;
