use crate::sdk::{order::Order, position::Position};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(get_all, frozen)]
#[derive(Debug, Clone)]
pub struct Action {
    pub desired_orders: HashMap<String, Order>,
    pub desired_positions: Vec<Position>,
}

#[pymethods]
impl Action {
    #[new]
    fn new(
        desired_orders: HashMap<String, Order>,
        desired_positions: Vec<Position>,
    ) -> PyResult<Action> {
        Ok(Action {
            desired_orders,
            desired_positions,
        })
    }
}
