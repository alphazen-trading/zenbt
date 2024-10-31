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
pub struct PySharedState {
    pub equity: Decimal,
    pub active_positions: Py<PyDict>,
    pub closed_positions: Py<PyDict>,
    pub active_position: Option<Py<Position>>,
    pub pending_limit_orders: Py<PyDict>,
}
#[pymethods]
impl PySharedState {}

// #[derive(Debug, Clone)]
// pub struct RustState {
//     pub equity: Vec<Decimal>,
//     pub floating_equity: Vec<Decimal>,
//     pub active_positions: HashMap<String, Position>,
//     pub closed_positions: HashMap<String, Position>,
// }
// impl RustState {}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct SharedState {
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub active_positions: HashMap<String, Position>,
    pub closed_positions: HashMap<String, Position>,
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
    // for key in py_actions.keys() {
    //     match key.as_str() {
    //         "new_position" => {
    //             if let Some(new_position) = py_actions.get(key).unwrap().downcast_ref::<Position>()
    //             {
    //                 // // println!("The new position is: {:?}", new_position);
    //                 // // Add position to the state safely
    //                 // set_state_dict_item(
    //                 //     _pystate,
    //                 //     "active_positions",
    //                 //     new_position.id.clone(),
    //                 //     new_position.clone().into_py(py), // Convert to a Python object
    //                 // );
    //                 // let mut pystate = _pystate.borrow_mut(py);
    //                 // pystate.active_position = Some(new_position.clone());
    //             } else {
    //                 // Handle the case where the downcast failed
    //                 println!("Error: The value associated with the key is not a `Position`.");
    //             }
    //         }
    //         "close_positions" => {
    //             // if let Some(positions_to_close) =
    //             //     py_actions.get(key).unwrap().downcast_ref::<Vec<String>>()
    //             // {
    //             //     // println!("The positions to close are: {:?}", positions_to_close);
    //             //     for pos_id in positions_to_close {
    //             //         remove_state_dict_item(_pystate, "active_positions", pos_id);
    //             //     }
    //             // }
    //         }
    //         _ => println!("Unknown key {:?}", key),
    //     }
    // }
    // append_decimal_to_list(_pystate, "equity", state.equity.last().unwrap().clone());

    let mut pystate = pystate.borrow_mut(py);
    pystate.equity = *state.equity.last().unwrap();
    if params.provide_active_position && !state.active_positions.is_empty() {
        let pos = state.active_positions.values().last().unwrap();
        pystate.active_position = Some(Py::new(py, pos.clone()).unwrap());
    };
}
