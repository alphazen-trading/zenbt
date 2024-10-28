use crate::sdk::{order::Order, position::Position};
use pyo3::prelude::*;
use std::collections::HashMap;

use pyo3::types::PyType;

#[pyclass(get_all, frozen)]
#[derive(Debug, Clone)]
pub struct Action {
    pub desired_orders: HashMap<String, Order>,
    pub desired_positions: HashMap<String, Position>,
}

#[pymethods]
impl Action {
    #[new]
    fn new(
        desired_orders: HashMap<String, Order>,
        desired_positions: HashMap<String, Position>,
    ) -> PyResult<Action> {
        Ok(Action {
            desired_orders,
            desired_positions,
        })
    }
}
