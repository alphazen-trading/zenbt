use crate::sdk::order::Order;
use pyo3::prelude::*;
use std::collections::HashMap;

/// Represents an action we want to place at the current cycle.
///
/// Usage:
///     ```python
///     from zenbt.zbt import (Order, Action, OrderType, Side)
///
///     order = Order(
///         index = 0,
///         client_order_id="OrderId",
///         order_type=OrderType.Market,
///         side=Side.Long,
///         size=1,
///         price=None,
///         sl=None,
///         tp=None
///     )
///
///     action = Action(
///         orders = {order.client_order_id: order},
///         close_all_positions=True
///     )
///
///     ```
///
/// Attributes:
///     orders (dict[str, Order]): A mapping of string keys to `Order` objects.
///     close_all_positions (bool): If True, backtester will close all open positions.

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
pub struct Action {
    pub orders: HashMap<String, Order>,
    pub close_all_positions: bool,
    // pub positions: HashMap<String, Position>,
    // pub position: Option<Position>,
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

    pub fn reset(&mut self) {
        self.orders = HashMap::new();
        self.close_all_positions = false;
        // self.positions = HashMap::new();
        // self.position = None;
    }
}
