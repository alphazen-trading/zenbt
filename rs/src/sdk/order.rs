use pyo3::prelude::*;
use rust_decimal::Decimal;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
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
