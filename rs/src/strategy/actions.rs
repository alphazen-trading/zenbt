use crate::sdk::order::Order;
use pyo3::prelude::*;
use std::collections::HashMap;

/// Desired Action to place
///
/// Usage:
///     ```python
///     action = Action()
///     ```

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
pub struct Action {
    /// A hashmap of orders you would like to engine to place
    pub orders: HashMap<String, Order>,
    // pub positions: HashMap<String, Position>,
    // pub position: Option<Position>,
    /// if True, backtester will close all open positions
    pub close_all_positions: bool,
}

#[pymethods]
impl Action {
    #[new]
    #[pyo3(signature = (orders, close_all_positions=false))]
    fn new(
        orders: HashMap<String, Order>,
        close_all_positions: bool,
        // positions: HashMap<String, Position>,
        // position: Option<Position>,
    ) -> Action {
        Action {
            orders,
            // positions,
            // position,
            close_all_positions,
        }
    }

    /// Method that does stuff
    pub fn reset(&mut self) {
        self.orders = HashMap::new();
        // self.positions = HashMap::new();
        // self.position = None;
        self.close_all_positions = false;
    }
}
