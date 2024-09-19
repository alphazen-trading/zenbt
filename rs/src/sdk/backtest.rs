use super::backtest_methods::{
    find_active_positions_to_close, find_signals_to_close, find_signals_to_enter,
    find_triggered_pending_orders, has_account_blown_up,
};

use super::backtest_params::BacktestParams;
use super::backtest_state::get_state;
use super::ohlc::{OHLCs, OHLC};
use super::order::LimitOrders;
use super::position::Positions;
use numpy::{PyArrayDyn, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::Bound;
use rust_decimal::Decimal;
// use super::signal::Signals;
use rust_decimal_macros::dec;

pub struct Signals {
    pub long_entries: PyArrayDyn<bool>,
    pub long_exits: PyArrayDyn<bool>,
    pub short_entries: PyArrayDyn<bool>,
    pub short_exits: PyArrayDyn<bool>,
}

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

    fn backtest_signals<'py>(
        &mut self,
        long_entries: &Bound<'py, PyArrayDyn<bool>>,
        long_exits: &Bound<'py, PyArrayDyn<bool>>,
        short_entries: &Bound<'py, PyArrayDyn<bool>>,
        short_exits: &Bound<'py, PyArrayDyn<bool>>,
    ) {
        let long_entries = unsafe { long_entries.as_array_mut() };
        let long_exits = unsafe { long_exits.as_array_mut() };
        let short_entries = unsafe { short_entries.as_array_mut() };
        let short_exits = unsafe { short_exits.as_array_mut() };

        for i in 0..self.ohlc.len() {
            find_signals_to_close(i, self, long_exits[i], short_exits[i]);

            // Now that we closed the positions, we check that indeed the account didn't blow
            if has_account_blown_up(&self.equity, &self.floating_equity) {
                println!("Account blew up");
                self.equity.pop();
                self.equity.push(dec!(0.0));
                break;
            }

            find_signals_to_enter(i, self, long_entries[i], short_entries[i]);
        }
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
            find_triggered_pending_orders(i, self);
        }
    }
}
