use pyo3::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;

#[pyclass]
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct BacktestParams {
    pub commission_pct: Decimal,
    pub initial_capital: Decimal,
}

#[pymethods]
impl BacktestParams {
    #[new]
    pub fn new(commission_pct: Decimal, initial_capital: Decimal) -> Self {
        BacktestParams {
            commission_pct,
            initial_capital,
        }
    }
}
