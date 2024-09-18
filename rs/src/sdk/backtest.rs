use super::backtest_methods::{
    find_active_positions_to_close, find_triggered_pending_orders, has_account_blown_up,
};
use super::backtest_params::BacktestParams;
use super::backtest_state::get_state;
use super::ohlc::{OHLCs, OHLC};
use super::order::LimitOrders;
use super::position::Positions;
use pyo3::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Backtest {
    pub ohlc: Vec<OHLC>,
    pub limit_orders: LimitOrders,
    pub trailing_tp: Vec<Decimal>,
    pub positions: Positions,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub commissions: Decimal,
    pub params: BacktestParams,
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Backtest {
    #[new]
    fn new(ohlcs: OHLCs, backtest_params: BacktestParams, limit_orders: LimitOrders) -> Self {
        Backtest {
            ohlc: ohlcs.ohlc,
            params: backtest_params,
            limit_orders,
            positions: Positions::new(),
            trailing_tp: Vec::new(),
            equity: Vec::new(),
            floating_equity: Vec::new(),
            commissions: dec![0],
        }
    }

    fn get_state(&self, py: Python) -> PyResult<PyObject> {
        get_state(self, py)
    }

    fn backtest(&mut self) {
        for i in 0..self.ohlc.len() {
            // We first need to check which positions will get closed
            find_active_positions_to_close(i, self);

            // Now that we closed the positions, we check that indeed the account didn't blow
            if has_account_blown_up(&self.equity, &self.floating_equity) {
                println!("Account blew up");
                self.equity.pop();
                self.equity.push(dec!(0.0));
                break;
            }

            // All good, we can check which of the pending orders got filled in that bar
            find_triggered_pending_orders(i, self)
        }
    }
}
