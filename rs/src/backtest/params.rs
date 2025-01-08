use pyo3::prelude::*;
use rust_decimal::Decimal;
use serde::Serialize;

#[pyclass]
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::module_name_repetitions)]
/// Parameters for the backtest.
///
/// Attributes:
///     initial_capital (decimal): The initial capital of the backtest
///     commission_pct (decimal): The commission percentage of the backtest
///     provide_active_position (bool): Whether to provide the active position to the strategy. If you don't need it, set this to false to make the backtest faster.
pub struct BacktestParams {
    pub verbose: bool,
    pub commission_pct: Decimal,
    pub initial_capital: Decimal,
    pub provide_active_position: bool,
}

#[pymethods]
impl BacktestParams {
    #[new]
    pub fn new(
        verbose: bool,
        commission_pct: Decimal,
        initial_capital: Decimal,
        provide_active_position: bool,
    ) -> Self {
        BacktestParams {
            verbose,
            commission_pct,
            initial_capital,
            provide_active_position,
        }
    }
}
