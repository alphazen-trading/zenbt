use crate::sdk::{enums::Side, order::Order};
use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

pub type LimitOrders = HashMap<Decimal, Vec<Order>>;

#[pyfunction]
pub fn create_limit_orders(limit_orders: Vec<Vec<f64>>) -> LimitOrders {
    let mut _limit_orders: LimitOrders = HashMap::new();

    for i in 0..limit_orders.len() {
        let index = Decimal::from_f64(limit_orders[i][0]).unwrap();
        if index != dec![0] {
            let side_decimal = Decimal::from_f64(limit_orders[i][1]).unwrap();

            #[allow(unused_variables)]
            let side = match side_decimal {
                d if d == dec!(1.0) => Side::Long,
                d if d == dec!(0.0) => Side::Short,
                _ => {
                    println!("Unknown side for value: {}", side_decimal);
                    continue;
                }
            };

            // let new_order = Order {
            //     index: limit_orders[i][0] as usize,
            //     order_type: OrderType::Limit,
            //     side,
            //     size: Decimal::from_f64(limit_orders[i][3]).unwrap(),
            //     price: Decimal::from_f64(limit_orders[i][2]).unwrap(),
            //     sl: Decimal::from_f64(limit_orders[i][4]).unwrap(),
            //     tp: Decimal::from_f64(limit_orders[i][5]).unwrap(),
            // };

            // match _limit_orders.get_mut(&index) {
            //     Some(vec) => vec.push(new_order),
            //     None => {
            //         let mut new_vec = Vec::new();
            //         new_vec.push(new_order);
            //         _limit_orders.insert(index, new_vec);
            //     }
            // }
        }
    }
    _limit_orders
}
