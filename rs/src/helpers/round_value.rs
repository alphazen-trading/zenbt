use pyo3::prelude::*;
use rust_decimal::Decimal;

#[pyfunction]
pub fn round_value(value: Decimal, step: Decimal) -> Decimal {
    (value / step).floor() * step
}
