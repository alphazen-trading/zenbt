use std::any::Any;
use std::collections::HashMap;

use crate::backtest::helpers::{remove_state_dict_item, set_state_dict_item};
use crate::sdk::position::Position;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rust_decimal::Decimal;

use super::helpers::{append_decimal_to_list, append_to_list};

#[pyclass(get_all)]
#[derive(Debug)]
pub struct PySharedState {
    pub equity: Py<PyList>,
    pub active_positions: Py<PyDict>,
    pub closed_positions: Py<PyDict>,
}
#[pymethods]
impl PySharedState {}

#[pyclass()]
#[derive(Debug, Clone)]
pub struct SharedState {
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub active_positions: HashMap<String, Position>,
    pub closed_positions: HashMap<String, Position>,
}
impl SharedState {}

// Function to copy values from `SharedState` to `PySharedState`
pub fn copy_shared_state_to_pystate(
    py: Python,
    copy_actions: HashMap<String, Box<dyn Any>>,
    state: &SharedState,
    _pystate: &Py<PySharedState>,
) {
    // let mut pystate = _pystate.borrow_mut(py);
    for key in copy_actions.keys() {
        match key.as_str() {
            "new_position" => {
                if let Some(new_position) =
                    copy_actions.get(key).unwrap().downcast_ref::<Position>()
                {
                    // Add position to the state safely
                    set_state_dict_item(
                        _pystate,
                        "active_positions",
                        new_position.id.clone(),
                        new_position.clone().into_py(py), // Convert to a Python object
                    );
                } else {
                    // Handle the case where the downcast failed
                    println!("Error: The value associated with the key is not a `Position`.");
                }
            }
            "close_positions" => {
                if let Some(positions_to_close) =
                    copy_actions.get(key).unwrap().downcast_ref::<Vec<String>>()
                {
                    for pos_id in positions_to_close {
                        remove_state_dict_item(_pystate, "active_positions", pos_id);
                    }
                }
            }
            _ => println!("Unknown key {:?}", key),
        }
    }
    append_decimal_to_list(_pystate, "equity", state.equity.last().unwrap().clone());
}
