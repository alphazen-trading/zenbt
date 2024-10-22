use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;

use super::enums::{OrderType, Side};

#[pyclass(get_all)]
#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub index: usize,
    pub order_type: OrderType,
    pub side: Side,
    pub size: Decimal,
    pub price: Option<Decimal>, // Optional price
    pub sl: Option<Decimal>,    // Optional stop-loss
    pub tp: Option<Decimal>,    // Optional take-profit
}

#[pymethods]
impl Order {
    #[new]
    fn new(
        index: usize,
        order_type: OrderType,
        side: Side,
        size: Decimal,
        price: Option<Decimal>, // Optional price
        sl: Option<Decimal>,    // Optional stop-loss
        tp: Option<Decimal>,    // Optional take-profit
    ) -> PyResult<Order> {
        Ok(Order {
            index,
            order_type,
            side,
            size,
            price,
            sl, // Keep sl as Option<Decimal>
            tp, // Keep tp as Option<Decimal>
        })
    }
    fn __repr__(&self) -> String {
        // Serialize the struct to a JSON string using serde_json
        match serde_json::to_string(self) {
            Ok(json_string) => json_string,
            Err(_) => "Failed to serialize Order struct".to_string(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct LimitOrders {
    pub limit_orders: HashMap<usize, Vec<Order>>,
}

#[pymethods]
impl LimitOrders {
    #[new]
    fn new(length: usize) -> LimitOrders {
        let mut limit_orders: HashMap<usize, Vec<Order>> = HashMap::new();
        for i in 0..length {
            limit_orders.insert(i, Vec::new());
        }

        LimitOrders { limit_orders }
    }

    pub fn create_order(
        &mut self,
        index: usize,
        order_type: OrderType,
        side: Side,
        price: Decimal,
        size: Decimal,
        sl: Decimal,
        tp: Decimal,
    ) {
        let order = Order {
            index,
            order_type,
            side,
            size,
            price: Some(price),
            sl: Some(sl),
            tp: Some(tp),
        };
        let vec = self
            .limit_orders
            .get_mut(&index)
            .expect("Index does not exist in limit_orders");
        vec.push(order);
    }

    pub fn get(&self, index: usize) -> Option<Order> {
        let vec = self
            .limit_orders
            .get(&index)
            .expect("Index does not exist in limit_orders");
        vec.last().cloned()
    }
    // pub fn get_limit_orders(&self) -> &HashMap<usize, Vec<Order>> {
    //     &self.limit_orders
    // }
}
