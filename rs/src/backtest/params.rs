use pyo3::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;

#[pyclass]
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct BacktestParams {
    pub commission_pct: Decimal,
    pub initial_capital: Decimal,
    pub provide_active_position: bool,
}

#[pymethods]
impl BacktestParams {
    #[new]
    pub fn new(
        commission_pct: Decimal,
        initial_capital: Decimal,
        provide_active_position: bool,
    ) -> Self {
        BacktestParams {
            commission_pct,
            initial_capital,
            provide_active_position,
        }
    }
}
