use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use rand::Rng;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;

use super::enums::{OrderStatus, OrderType, Side};

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone, Serialize)]
/// Object that represents an order in the system
///
/// Attributes:
///     index (int): The index of the order
///     client_order_id (str): The client order ID
///     order_type (OrderType): The type of the order
///     side (Side): The side of the order
///     size (Decimal): The size of the order
///     price (Decimal): The price of the order
///     sl (Decimal): The stop-loss of the order
///     tp (Decimal): The take-profit of the order
pub struct Order {
    pub id: String,
    pub index: usize,
    pub place_timestamp: DateTime<Utc>,
    pub fill_timestamp: Option<DateTime<Utc>>,
    pub status: OrderStatus,
    pub client_order_id: String,
    #[allow(clippy::struct_field_names)]
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
    #[allow(clippy::similar_names, clippy::too_many_arguments)]
    #[pyo3(signature = (index, client_order_id, order_type, place_timestamp, fill_timestamp, status, side, size, price=None, sl=None, tp=None))]
    fn new(
        index: usize,
        client_order_id: String,
        order_type: OrderType,
        place_timestamp: DateTime<Utc>,
        fill_timestamp: Option<DateTime<Utc>>,
        status: OrderStatus,
        side: Side,
        size: Decimal,
        price: Option<Decimal>, // Optional price
        sl: Option<Decimal>,    // Optional stop-loss
        tp: Option<Decimal>,    // Optional take-profit
    ) -> Order {
        Order {
            id: rand::thread_rng().gen_range(0..999_999_999).to_string(),
            index,
            place_timestamp,
            fill_timestamp,
            status,
            client_order_id,
            order_type,
            side,
            size,
            price,
            sl,
            tp,
        }
    }
    fn __repr__(&self) -> String {
        // Serialize the struct to a JSON string using serde_json
        match serde_json::to_string(self) {
            Ok(json_string) => json_string,
            Err(_) => "Failed to serialize Order struct".to_string(),
        }
    }
}

// #[pyclass]
// #[derive(Debug, Clone)]
// pub struct LimitOrders {
//     pub limit_orders: HashMap<usize, Vec<Order>>,
// }

// #[pymethods]
// impl LimitOrders {
//     #[new]
//     fn new(length: usize) -> LimitOrders {
//         let mut limit_orders: HashMap<usize, Vec<Order>> = HashMap::new();
//         for i in 0..length {
//             limit_orders.insert(i, Vec::new());
//         }

//         LimitOrders { limit_orders }
//     }

//     #[allow(clippy::similar_names, clippy::too_many_arguments)]
//     pub fn create_order(
//         &mut self,
//         index: usize,
//         order_type: OrderType,
//         side: Side,
//         size: Decimal,
//         price: Decimal,
//         sl: Decimal,
//         tp: Decimal,
//     ) {
//         let order = Order {
//             index,
//             client_order_id: String::new(),
//             order_type,
//             side,
//             size,
//             price: Some(price),
//             sl: Some(sl),
//             tp: Some(tp),
//         };
//         let vec = self
//             .limit_orders
//             .get_mut(&index)
//             .expect("Index does not exist in limit_orders");
//         vec.push(order);
//     }

//     pub fn get(&self, index: usize) -> Option<Order> {
//         let vec = self
//             .limit_orders
//             .get(&index)
//             .expect("Index does not exist in limit_orders");
//         vec.last().cloned()
//     }
//     // pub fn get_limit_orders(&self) -> &HashMap<usize, Vec<Order>> {
//     //     &self.limit_orders
//     // }
// }
