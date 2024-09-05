use super::ohlc::OHLC;
use super::order::Order;
use super::position::Position;
use chrono::{DateTime, TimeZone, Utc};
use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Div;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Backtest {
    pub ohlc: Vec<OHLC>,
    pub limit_orders: HashMap<Decimal, Vec<Order>>,
    pub trailing_tp: Vec<Decimal>,
    pub active_positions: Vec<Position>,
    pub closed_positions: Vec<Position>,
    pub commission: Decimal,
}
#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Backtest {
    #[new]
    fn new(data: Vec<Vec<f64>>, commission: Decimal) -> Self {
        let mut ohlcs = Vec::new();
        let mut _limit_orders: HashMap<Decimal, Vec<Order>> = HashMap::new();

        for i in 0..data.len() {
            let ohlc = OHLC {
                date: DateTime::from_timestamp(data[i][0].to_i64().unwrap().div(1000), 0)
                    .expect("Invalid timestamp"),
                open: Decimal::from_f64(data[i][1]).unwrap(),
                high: Decimal::from_f64(data[i][2]).unwrap(),
                low: Decimal::from_f64(data[i][3]).unwrap(),
                close: Decimal::from_f64(data[i][4]).unwrap(),
                volume: Decimal::from_f64(data[i][5]).unwrap(),
            };
            ohlcs.push(ohlc);
        }
        Backtest {
            ohlc: ohlcs,
            limit_orders: HashMap::new(),
            active_positions: Vec::new(),
            closed_positions: Vec::new(),
            trailing_tp: Vec::new(),
            commission,
        }
    }
    fn prepare(&mut self, limit_orders: Vec<Vec<f64>>, trailing_tp: Vec<f64>) {
        let mut _trailing_tp = Vec::new();
        let mut _limit_orders: HashMap<Decimal, Vec<Order>> = HashMap::new();

        for i in 0..trailing_tp.len() {
            _trailing_tp.push(Decimal::from_f64(trailing_tp[i]).unwrap_or(dec![0]));
        }
        for i in 0..limit_orders.len() {
            let index = Decimal::from_f64(limit_orders[i][0]).unwrap();
            if index != dec![0] {
                let new_order = Order {
                    index: index,
                    side: Decimal::from_f64(limit_orders[i][1]).unwrap(),
                    price: Decimal::from_f64(limit_orders[i][2]).unwrap(),
                    size: Decimal::from_f64(limit_orders[i][3]).unwrap(),
                    sl: Decimal::from_f64(limit_orders[i][4]).unwrap(),
                    tp: Decimal::from_f64(limit_orders[i][5]).unwrap(),
                    order_type: String::from("limit"),
                };

                match _limit_orders.get_mut(&index) {
                    Some(vec) => vec.push(new_order),
                    None => {
                        let mut new_vec = Vec::new();
                        new_vec.push(new_order);
                        _limit_orders.insert(index, new_vec);
                    }
                }
            }
        }
        self.limit_orders = _limit_orders;
        self.trailing_tp = _trailing_tp;
        self.active_positions = Vec::new();
        self.closed_positions = Vec::new();
    }

    fn print(&self) {
        println!("{:?}", self.ohlc);
    }

    fn was_order_hit(&self, ohlc: &OHLC, order: &Order) -> PyResult<bool> {
        if order.side == dec!(1.0) {
            // if ohlc.low <= order.price {
            //     println!("ORDER WAS HIT");
            //     println!("{:?}", ohlc.low);
            //     println!("{:?}", order.price);
            // }
            return Ok(ohlc.low <= order.price);
        } else {
            // if ohlc.high >= order.price {
            //     println!("ORDER WAS HIT");
            //     println!("{:?}", ohlc);
            //     println!("{:?}", order);
            // }
            return Ok(ohlc.high >= order.price);
        }
    }
    fn backtest(&mut self) {
        for i in 0..self.ohlc.len() {
            let ohlc = &self.ohlc[i];
            let orders = self.limit_orders.get(&Decimal::from(i));
            // println!("------------");

            let mut indexes_to_remove = Vec::new();
            for (j, position) in &mut self.active_positions.iter_mut().enumerate() {
                let should = position.should_close(&ohlc);
                if should {
                    self.closed_positions.push(position.clone());
                    indexes_to_remove.push(j);
                } else {
                    // position.tp = self.trailing_tp[i];
                    position.update_pnl(ohlc.close);
                }
            }
            for &i in indexes_to_remove.iter().rev() {
                self.active_positions.remove(i);
            }

            if orders.is_none() {
                continue;
            }
            // println!("{:?}", orders.unwrap().len());
            // println!("{:?}", i);
            for order in orders.unwrap() {
                let was_hit = self.was_order_hit(&ohlc, &order);
                match was_hit {
                    Ok(true) => {
                        // println!("ORDER WAS HIT");
                        // println!("{:?}", order);
                        // println!("{:?}", self.ohlc[i - 1]);
                        self.active_positions.push(Position {
                            index: order.index,
                            entry_timestamp: ohlc.date,
                            exit_timestamp: None,
                            entry_price: order.price,
                            exit_price: None,
                            size: order.size,
                            sl: order.sl,
                            tp: order.tp,
                            side: order.side,
                            close_reason: None,
                            pnl: dec!(0.0),
                            max_dd: dec!(0.0),
                            commission: order.price * self.commission * order.size,
                            commission_pct: self.commission,
                        });
                        // println!("{:?}", self.active_positions);
                        // println!("{:?}", was_hit);
                        // println!("{:?}", order);
                        // println!("{:?}\n", ohlc);
                    }
                    _ => {}
                }
            }
        }
    }

    #[getter]
    fn closed_positions(&self) -> PyResult<Vec<Position>> {
        Ok(self.closed_positions.clone())
    }

    #[getter]
    fn active_positions(&self) -> PyResult<Vec<Position>> {
        Ok(self.active_positions.clone())
    }
}
