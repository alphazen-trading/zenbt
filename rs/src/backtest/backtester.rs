use super::helpers::{get_date_at_index, get_value_at};
use super::methods::check_positions_to_close;
use super::params::BacktestParams;
use super::shared_state::{copy_shared_state_to_pystate, PySharedState, SharedState};
use super::stats_methods::create_stats;
use crate::sdk::enums::OrderType;
use crate::sdk::position::Position;
use crate::strategy::actions::Action;
use crate::strategy::base::Strategy;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

use pyo3_polars::PyDataFrame;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Backtest {
    pub df: PyDataFrame,
    pub data: Py<PyDict>,
    pub params: BacktestParams,
    pub strategy: Py<Strategy>,
    pub state: SharedState,
    pub pystate: Py<PySharedState>,
    pub commissions: Decimal,
}

#[pymethods]
impl Backtest {
    #[new]
    fn new(df: PyDataFrame, params: BacktestParams, strategy: Py<Strategy>) -> PyResult<Backtest> {
        Python::with_gil(|py| {
            let data = strategy.borrow(py).data.clone_ref(py);
            let pyequity = PyList::new_bound(py, Vec::<f64>::new());
            pyequity.append(params.initial_capital).unwrap();

            let pystate = Py::new(
                py,
                PySharedState {
                    equity: params.initial_capital,
                    active_positions: PyDict::new_bound(py).into(),
                    closed_positions: PyDict::new_bound(py).into(),
                    active_position: None,
                },
            )?;
            let equity = vec![params.initial_capital];

            Ok(Backtest {
                df,
                data,
                params,
                strategy,
                pystate,
                state: SharedState {
                    equity,
                    floating_equity: Vec::new(),
                    active_positions: HashMap::new(),
                    closed_positions: HashMap::new(),
                },
                commissions: dec!(0),
            })
        })
    }

    fn backtest(&mut self) {
        let df = self.df.0.clone();
        for i in 0..df.height() {
            let mut action = Python::with_gil(|py| {
                let result: Py<Action> = self
                    .strategy
                    .call_method_bound(
                        py,
                        intern!(py, "on_candle"),
                        // (i, self.pystate.borrow(py)),
                        (),
                        None,
                    )
                    .unwrap()
                    .extract(py)
                    .unwrap();
                result.extract::<Action>(py).unwrap()
            });

            // let mut py_actions: HashMap<String, Box<dyn Any>> = HashMap::new();
            check_positions_to_close(i, &df, self, &action);

            for order in action.orders.values_mut() {
                if order.order_type == OrderType::Market {
                    order.price = Some(get_value_at(&df, i + 1, "open"));
                    let new_position =
                        Position::create_position(order, get_date_at_index(&df, i), &self.params);
                    self.state
                        .active_positions
                        .insert(new_position.id.clone(), new_position.clone());

                    // py_actions.insert("new_position".to_string(), Box::new(new_position));
                }
                Python::with_gil(|py| {
                    self.strategy
                        .call_method_bound(py, intern!(py, "reset_action"), (), None)
                        .unwrap();
                });
            }
            // Usage example
            Python::with_gil(|py| {
                copy_shared_state_to_pystate(py, &self.state, &self.pystate, &self.params);
            });
        }
    }

    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("stats", create_stats(self))?;

        Ok(dict.into())
    }
}
