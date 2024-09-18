use super::backtest_methods::{
    find_active_positions_to_close, find_triggered_pending_orders, has_account_blown_up,
};
use super::backtest_params::BacktestParams;
use super::ohlc::{OHLCs, OHLC};
use super::order::LimitOrders;
use super::position::Positions;
use super::stats::create_stats;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone, Serialize)]
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

    // Method that returns the data as a Python dictionary
    fn get_data_as_dict(&self, py: Python) -> PyResult<PyObject> {
        // Create a new PyDict
        let dict = PyDict::new_bound(py);

        // Insert the struct's fields into the PyDict
        dict.set_item("commission_pct", self.params.commission_pct)?;
        dict.set_item("commissions", self.commissions)?;
        dict.set_item("initial_capital", self.params.initial_capital)?;
        dict.set_item("ohlc", self.ohlc.clone())?;
        dict.set_item("active_positions", self.positions.active_positions.clone())?;
        dict.set_item("closed_positions", self.positions.closed_positions.clone())?;
        dict.set_item("equity", self.equity.clone())?;
        dict.set_item("floating_equity", self.floating_equity.clone())?;

        Ok(dict.into())
    }

    #[getter]
    fn stats(&self) -> PyResult<String> {
        match serde_json::to_string(&create_stats(&self)) {
            Ok(json_string) => Ok(json_string),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(err.to_string())),
        }
    }
}
