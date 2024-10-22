use super::backtest_params::BacktestParams;
use super::shared_state::SharedState;

use crate::sdk::position::Positions;
use crate::strategy::actions::Action;
use crate::strategy::strategy::Strategy;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyList;

// use super::ohlc::OHLC;
use pyo3_polars::PyDataFrame;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::borrow::BorrowMut;

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Backtest {
    pub df: PyDataFrame,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub backtest_params: BacktestParams,
    pub positions: Positions,
    pub strategy: Py<Strategy>,
    pub has_position: bool,
    pub state: Py<SharedState>,
}

#[pymethods]
impl Backtest {
    #[new]
    fn new(
        df: PyDataFrame,
        backtest_params: BacktestParams,
        strategy: Py<Strategy>,
    ) -> PyResult<Backtest> {
        Python::with_gil(|py| {
            Ok(Backtest {
                df,
                equity: Vec::new(),
                floating_equity: Vec::new(),
                backtest_params,
                positions: Positions::new(),
                strategy,
                has_position: false,
                state: Py::new(
                    py,
                    SharedState {
                        equity: PyList::new_bound(py, Vec::<f64>::new()).into(),
                    },
                )?,
            })
        })
    }

    fn _append_to_list(&self, list_name: &str, value: Decimal) {
        Python::with_gil(|py| {
            let mut list = self.state.getattr(py, list_name).unwrap();
            list.borrow_mut()
                .call_method_bound(py, "append", (value,), None)
                .unwrap();
        });
    }

    fn backtest(&self) {
        let df = self.df.0.clone();
        for i in 0..df.height() {
            self._append_to_list("equity", Decimal::from_usize(i).unwrap());
            Python::with_gil(|py| {
                let state = self.state.borrow(py);
                let result: Py<Action> = self
                    .strategy
                    // .call_method_bound(py, "on_candle", (i, self.has_position), Some(&kwargs_dict))
                    .call_method_bound(py, intern!(py, "on_candle"), (i, state), None)
                    .unwrap()
                    .extract(py)
                    .unwrap();
                let action = result.extract::<Action>(py).unwrap();
                // println!("Za val is this {:?}", action);
            });
        }
    }
}
