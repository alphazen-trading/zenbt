use crate::sdk::{order::Order, position::Position};
use pyo3::prelude::*;

#[pyclass(get_all, frozen)]
#[derive(Debug, Clone)]
pub struct Action {
    pub desired_orders: Vec<Order>,
    pub desired_positions: Vec<Position>,
}

#[pymethods]
impl Action {
    #[new]
    fn new(desired_orders: Vec<Order>, desired_positions: Vec<Position>) -> PyResult<Action> {
        Ok(Action {
            desired_orders,
            desired_positions,
        })
    }
}
