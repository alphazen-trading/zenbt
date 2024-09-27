use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;

use super::enums::{OrderType, Side};

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Order {
    pub index: usize,
    pub order_type: OrderType,
    pub side: Side,
    pub price: Decimal,
    pub size: Decimal,
    pub sl: Decimal,
    pub tp: Decimal,
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct LimitOrders {
    pub limit_orders: HashMap<usize, Vec<Order>>,
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
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
        price: f64,
        size: f64,
        sl: f64,
        tp: f64,
    ) {
        let order = Order {
            index,
            order_type,
            side,
            price: Decimal::from_f64(price).unwrap(),
            size: Decimal::from_f64(size).unwrap(),
            sl: Decimal::from_f64(sl).unwrap(),
            tp: Decimal::from_f64(tp).unwrap(),
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
