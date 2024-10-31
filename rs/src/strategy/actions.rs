use crate::sdk::{order::Order, position::Position};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
pub struct Action {
    pub orders: HashMap<String, Order>,
    // pub positions: HashMap<String, Position>,
    pub position: Option<Position>,
    pub close_all_positions: bool,
}

#[pymethods]
impl Action {
    #[new]
    #[pyo3(signature = (orders, position=None, close_all_positions=false))]
    fn new(
        orders: HashMap<String, Order>,
        // positions: HashMap<String, Position>,
        position: Option<Position>,
        close_all_positions: bool,
    ) -> Action {
        Action {
            orders,
            // positions,
            position,
            close_all_positions,
        }
    }

    pub fn reset(&mut self) {
        self.orders = HashMap::new();
        // self.positions = HashMap::new();
        self.position = None;
        self.close_all_positions = false;
    }
}
