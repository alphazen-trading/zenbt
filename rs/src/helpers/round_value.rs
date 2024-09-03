use pyo3::prelude::*;
use rust_decimal::Decimal;

#[pyfunction]
pub fn round_value(value: Decimal, step: Decimal) -> Decimal {
    let res = (value / step).floor() * step;
    res
}
