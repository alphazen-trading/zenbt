use pyo3::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub index: Decimal,
    pub price: Decimal,
    pub size: Decimal,
    pub sl: Decimal,
    pub tp: Decimal,
    pub side: Decimal,
    pub order_type: String,
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Order {
    #[getter]
    fn print(&self) {
        println!("{:?}", self);
    }
}

pub type LimitOrders = HashMap<Decimal, Vec<Order>>;
