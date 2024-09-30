use super::backtest_methods::{
    find_active_positions_to_close, find_signals_to_close, find_signals_to_enter,
    find_triggered_pending_orders, has_account_blown_up,
};
use super::stats_methods::create_stats;
use pyo3::intern;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use std::fs;
use std::path::Path;

use super::backtest_params::BacktestParams;
use super::backtest_state::get_state;
use super::ohlc::{OHLCs, OHLC};
use super::order::LimitOrders;
use super::position::Positions;
// use super::signal::Signals;
use pyo3::prelude::*;
// use pyo3::Bound;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass]
#[derive(Debug)]
pub struct VAL {
    #[pyo3(get)]
    pub curr: f64,
}

#[pyclass(get_all)]
#[derive(Debug)]
pub struct Backtest {
    pub ohlc: Vec<OHLC>,
    pub limit_orders: LimitOrders,
    pub trailing_tp: Vec<Decimal>,
    pub positions: Positions,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub commissions: Decimal,
    pub params: BacktestParams,
    pub external: Py<VAL>,
}

#[pymethods]
impl Backtest {
    #[new]
    fn new(
        ohlcs: OHLCs,
        backtest_params: BacktestParams,
        limit_orders: LimitOrders,
    ) -> PyResult<Backtest> {
        Python::with_gil(|py| {
            let external: Py<VAL> = Py::new(py, VAL { curr: 0.0 })?;
            Ok(Backtest {
                ohlc: ohlcs.ohlc,
                params: backtest_params,
                limit_orders,
                positions: Positions::new(),
                trailing_tp: Vec::new(),
                equity: Vec::new(),
                floating_equity: Vec::new(),
                commissions: dec![0],
                external,
            })
        })
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("stats", create_stats(&self))?;

        Ok(dict.into())
    }

    fn get_state(&self, py: Python) -> PyResult<PyObject> {
        get_state(self, py)
    }

    // fn backtest_signals<'py>(
    //     &mut self,
    //     long_entries: &Bound<'py, PyArrayDyn<bool>>,
    //     long_exits: &Bound<'py, PyArrayDyn<bool>>,
    //     short_entries: &Bound<'py, PyArrayDyn<bool>>,
    //     short_exits: &Bound<'py, PyArrayDyn<bool>>,
    // ) {
    //     let long_entries = unsafe { long_entries.as_array_mut() };
    //     let long_exits = unsafe { long_exits.as_array_mut() };
    //     let short_entries = unsafe { short_entries.as_array_mut() };
    //     let short_exits = unsafe { short_exits.as_array_mut() };

    //     for i in 0..self.ohlc.len() {
    //         find_signals_to_close(i, self, long_exits[i], short_exits[i]);

    //         // Now that we closed the positions, we check that indeed the account didn't blow
    //         if has_account_blown_up(&self.equity, &self.floating_equity) {
    //             println!("Account blew up");
    //             self.equity.pop();
    //             self.equity.push(dec!(0.0));
    //             break;
    //         }

    //         find_signals_to_enter(i, self, long_entries[i], short_entries[i]);
    //     }
    // }

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
    fn backtest_with_cb(&mut self) {
        let path = Path::new("/home/alpha/workspace/alphazen/zenbt/src/strategy/");
        let py_app = fs::read_to_string(path.join("app.py")).unwrap();

        // let code: Bound<'_, PyAny> = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let code: Py<PyAny> = Python::with_gil(|py| {
            // let code = Python::with_gil(|py| {
            let syspath = py
                .import_bound("sys")
                .unwrap()
                .getattr("path")
                .unwrap()
                .downcast_into::<PyList>()
                .unwrap();
            syspath.insert(0, &path).unwrap();

            let strategy_class = PyModule::from_code_bound(py, &py_app, "", "")
                .unwrap()
                .getattr("Strategy")
                .unwrap();

            let house = strategy_class.call0().unwrap();
            // house.call_method1("__enter__", self).unwrap();

            // let dict = PyDict::new_bound(py);
            // dict.set_item("equity", 1.0);
            house.call_method1("major", (3,)).unwrap();
            // let test: Bound<'_, VAL> = VAL { curr: 4.0 }.into_py_dict_bound(py);

            strategy_class.call0().unwrap();
            strategy_class.into()
        });
        println!("Code: {:?}", code);

        for i in 0..self.ohlc.len() {
            let val = VAL { curr: i as f64 };
            let realized_equity = dec!(0);
            let from_python = Python::with_gil(|py| {
                // let kwargs = [("equity", 2)].into_py_dict_bound(py);
                let dict = PyDict::new_bound(py);
                self.equity.push(
                    self.equity.last().unwrap_or(&self.params.initial_capital) + realized_equity,
                );

                dict.set_item("equity", self.equity.last()).unwrap();

                let res: i32 = code
                    .call_method_bound(py, intern!(py, "major"), ("",), Some(&dict))
                    .unwrap()
                    .extract(py)
                    .unwrap();
            });
            // println!("Hello world {:?}", res);
        }
        // let dict = PyDict::new_bound(py);
        // dict.set_item("equity", self.equity.last())?;
        // code.call_method1(py, (dict,))
        // match from_python {
        //     Ok(_) => Ok(()),
        //     Err(e) => {
        //         println!("ERROR {:?}", e);
        //         Err(e)
        //     }
        // }
        // println!("DONE {:?}", from_python);
        // Ok(())
    }
}
