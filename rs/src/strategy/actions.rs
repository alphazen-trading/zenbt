use crate::sdk::{order::Order, position::Position};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(get_all, frozen)]
#[derive(Debug, Clone)]
pub struct Action {
    pub orders: HashMap<String, Order>,
    pub positions: HashMap<String, Position>,
    pub position: Option<Position>,
}

#[pymethods]
impl Action {
    #[new]
    #[pyo3(signature = (orders, positions, position=None))]
    fn new(
        orders: HashMap<String, Order>,
        positions: HashMap<String, Position>,
        position: Option<Position>,
    ) -> Action {
        Action {
            orders,
            positions,
            position,
        }
    }
}
