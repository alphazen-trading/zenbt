use crate::sdk::order::Order;
use crate::sdk::position::Position;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::params::BacktestParams;

#[pyclass(get_all)]
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
/// The shared state between python and the backtest.
/// This one get passed back to the Strategy on the oncandle method.
///
/// Attributes:
///     equity (Decimal): The current equity of the backtest
///     active_positions (dict[str, Position]): A mapping of string keys to `Position` objects.
///     closed_positions (dict[str, Position]): A mapping of string keys to `Position` objects.
///     active_position (Option[Position]): The current active position of the backtest
///     pending_limit_orders (dict[str, Order]): A mapping of string keys to `Order` objects.
pub struct PySharedState {
    pub equity: Decimal,
    pub active_positions: Py<PyDict>,
    pub closed_positions: Py<PyDict>,
    pub active_position: Option<Py<Position>>,
    pub pending_limit_orders: Py<PyDict>,
}
#[pymethods]
impl PySharedState {}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
/// The rust shared state, used internally by the backtester
///
/// Attributes:
///     equity (Decimal): The current equity of the backtest
///     floating_equity (Decimal): The current floating equity of the backtest
///     active_positions (dict[str, Position]): A mapping of string keys to `Position` objects.
///     closed_positions (dict[str, Position]): A mapping of string keys to `Position` objects.
///     pending_limit_orders (dict[str, Order]): A mapping of string keys to `Order` objects.
pub struct SharedState {
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub active_positions: HashMap<String, Position>,
    pub closed_positions: HashMap<String, Position>,
    pub orders: HashMap<String, Order>,
    pub pending_limit_orders: HashMap<String, Order>,
}
impl SharedState {}

// Function to copy values from `SharedState` to `PySharedState`
pub fn copy_shared_state_to_pystate(
    py: Python,
    // i: usize,
    // py_actions: HashMap<String, Box<dyn Any>>,
    state: &SharedState,
    pystate: &Py<PySharedState>,
    params: &BacktestParams,
) {
    let mut pystate = pystate.borrow_mut(py);
    pystate.equity = *state.equity.last().unwrap();
    if params.provide_active_position {
        if state.active_positions.is_empty() {
            pystate.active_position = None;
        } else {
            let pos = state.active_positions.values().last().unwrap();
            pystate.active_position = Some(Py::new(py, pos.clone()).unwrap());
        }
    };
}
