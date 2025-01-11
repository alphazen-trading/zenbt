use super::helpers::{get_date_at_index, get_value_at};
use super::methods::{check_positions_to_close, was_pending_order_triggered};
use super::params::BacktestParams;
use super::shared_state::{copy_shared_state_to_pystate, PySharedState, SharedState};
use super::stats_methods::create_stats;
use crate::sdk::enums::{OrderStatus, OrderType};
use crate::sdk::order::Order;
use crate::sdk::position::Position;
use crate::strategy::actions::Action;
use crate::strategy::base::Strategy;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

use pyo3_polars::PyDataFrame;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass(get_all, set_all, subclass)]
#[derive(Debug)]
/// A class that will run a backtest and generate stats as a result.
///
/// Attributes:
///     df (PyDataFrame): The dataframe of the backtest
///     data (PyDict): The data of the strategy
///     params (BacktestParams): The params of the backtest
///     strategy (Strategy): The strategy that will be backtested
///     state (SharedState): The shared state of the backtest
///     pystate (PySharedState): The shared state of the backtest
///     commissions (Decimal): The commissions of the backtest
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
                    pending_limit_orders: PyDict::new_bound(py).into(),
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
                    orders: Vec::new(),
                    pending_limit_orders: HashMap::new(),
                },
                commissions: dec!(0),
            })
        })
    }

    /// Run a backtest given the params and settings passed to the object
    fn backtest(&mut self) {
        let df = self.df.0.clone();
        // let equity = vec![self.params.initial_capital];
        // let mut state = RustState {
        //     equity,
        //     floating_equity: Vec::new(),
        //     active_positions: HashMap::new(),
        //     closed_positions: HashMap::new(),
        // };
        for i in 0..df.height() {
            // let mut action = self.strategy.get().fast_method_test(i, &df);
            let mut action = Python::with_gil(|py| {
                let result: Py<Action> = self
                    .strategy
                    .call_method_bound(
                        py,
                        intern!(py, "_on_candle"),
                        (self.pystate.borrow(py),),
                        // (),
                        None,
                    )
                    .unwrap()
                    .extract(py)
                    .unwrap();
                result.extract::<Action>(py).unwrap()
            });

            // check_positions_to_close(i, &df, self, &action, &mut state);
            check_positions_to_close(i, &df, self, &action);

            let mut filled_pending_orders: Vec<Order> = Vec::new();
            let mut pending_limit_orders = self.state.pending_limit_orders.clone(); // Clone here to avoid borrowing issues
            for pending_order in pending_limit_orders.values_mut() {
                if was_pending_order_triggered(pending_order, i, &df, self) {
                    if self.params.verbose {
                        println!("{i} Triggered order: {:?}", pending_order.clone());
                    }
                    filled_pending_orders.push(pending_order.clone());
                }
            }
            for order in filled_pending_orders {
                self.state
                    .pending_limit_orders
                    .remove(&order.client_order_id);
            }

            if action.cancel_pending_orders {
                self.state.pending_limit_orders.clear();
            }

            // Add orders from the position into here
            for order in action.orders.values_mut() {
                if self.params.verbose {
                    println!("{i} Placing a {:?} order: {:?}", order.order_type, order);
                }

                match order.order_type {
                    OrderType::Market => {
                        order.price = Some(get_value_at(&df, i + 1, "open"));
                        let new_position = Position::create_position(
                            order,
                            get_date_at_index(&df, i),
                            &self.params,
                        );
                        self.state
                            .active_positions
                            .insert(new_position.id.clone(), new_position);

                        order.status = OrderStatus::Filled;
                        self.state.orders.push(order.clone());
                    }
                    OrderType::Limit | OrderType::Stop => {
                        if !was_pending_order_triggered(order, i, &df, self) {
                            self.state
                                .pending_limit_orders
                                .insert(order.client_order_id.clone(), order.clone());
                        }
                    }
                }

                // Python::with_gil(|py| {
                //     self.strategy
                //         .call_method_bound(py, intern!(py, "reset_action"), (), None)
                //         .unwrap();
                // });
            }

            // Usage example
            Python::with_gil(|py| {
                copy_shared_state_to_pystate(py, &self.state, &self.pystate, &self.params);
            });
            // println!("{i} -----------------");
            // println!("{:?}", self.state.pending_limit_orders);
        }
    }

    /// Method that will return stats of the backtest
    ///
    /// Returns:
    ///     stats (Stats): Dictionary with stats of the backtest
    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("stats", create_stats(self))?;

        Ok(dict.into())
    }
}
