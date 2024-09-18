use super::backtest_methods::{create_position, has_account_blown_up, was_order_hit};
use super::backtest_params::BacktestParams;
use super::ohlc::OHLC;
use super::order::Order;
use super::position::Position;
use super::stats::create_stats;
use chrono::{DateTime, TimeZone, Utc};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Div;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone, Serialize)]
pub struct Backtest {
    pub ohlc: Vec<OHLC>,
    pub limit_orders: HashMap<Decimal, Vec<Order>>,
    pub trailing_tp: Vec<Decimal>,
    pub active_positions: Vec<Position>,
    pub closed_positions: Vec<Position>,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub commissions: Decimal,
    pub params: BacktestParams,
}
#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Backtest {
    #[new]
    fn new(data: Vec<Vec<f64>>, backtest_params: BacktestParams) -> Self {
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
            equity: Vec::new(),
            floating_equity: Vec::new(),
            commissions: dec![0],
            params: backtest_params,
        }
    }
    fn reset(&mut self) {
        self.trailing_tp = Vec::new();
        self.active_positions = Vec::new();
        self.closed_positions = Vec::new();
        self.commissions = dec![0];
        self.equity = Vec::new();
        self.floating_equity = Vec::new();
    }
    fn prepare(&mut self, limit_orders: Vec<Vec<f64>>) {
        // let mut _trailing_tp = Vec::new();
        let mut _limit_orders: HashMap<Decimal, Vec<Order>> = HashMap::new();

        // for i in 0..trailing_tp.len() {
        //     _trailing_tp.push(Decimal::from_f64(trailing_tp[i]).unwrap_or(dec![0]));
        // }
        for i in 0..limit_orders.len() {
            let index = Decimal::from_f64(limit_orders[i][0]).unwrap();
            if index != dec![0] {
                let new_order = Order {
                    index,
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
        // self.trailing_tp = _trailing_tp;
        self.reset()
    }

    fn print(&self) {
        println!("{:?}", self.ohlc);
    }

    fn backtest(&mut self) {
        for i in 0..self.ohlc.len() {
            let ohlc = &self.ohlc[i];
            let mut indexes_to_remove = Vec::new();
            let mut floating_equity = dec!(0);
            let mut realized_equity = dec!(0);

            for (j, position) in &mut self.active_positions.iter_mut().enumerate() {
                let should = position.should_close(&ohlc);
                if should {
                    self.closed_positions.push(position.clone());
                    self.commissions += position.commission;
                    realized_equity += position.pnl;
                    indexes_to_remove.push(j);
                } else {
                    // position.tp = self.trailing_tp[i];
                    position.update_pnl(ohlc.close);
                    floating_equity += position.pnl;
                }
            }
            self.floating_equity.push(floating_equity);
            self.equity
                .push(self.equity.last().unwrap_or(&self.params.initial_capital) + realized_equity);

            for &i in indexes_to_remove.iter().rev() {
                self.active_positions.remove(i);
            }

            if has_account_blown_up(&self.equity, &self.floating_equity) {
                println!("Account blew up");
                self.equity.pop();
                self.equity.push(dec!(0.0));
                break;
            }

            let orders = self.limit_orders.get(&Decimal::from(i));
            if orders.is_some() {
                for order in orders.unwrap() {
                    let was_hit = was_order_hit(&ohlc, &order);
                    match was_hit {
                        true => {
                            let mut new_position = create_position(&order, ohlc, &self.params);
                            if new_position.was_sl_hit(&ohlc) {
                                // println!("SL HIT in the same candle");
                                self.closed_positions.push(new_position);
                            } else {
                                self.active_positions.push(new_position);
                            }
                        }
                        _ => {}
                    }
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

    #[getter]
    fn equity(&self) -> PyResult<Vec<Decimal>> {
        Ok(self.equity.clone())
    }
    #[getter]
    fn floating_equity(&self) -> PyResult<Vec<Decimal>> {
        Ok(self.floating_equity.clone())
    }
    // Method that returns the data as a Python dictionary
    fn get_data_as_dict(&self, py: Python) -> PyResult<PyObject> {
        let mut data = HashMap::new();
        data.insert("key1".to_string(), 10);
        data.insert("key2".to_string(), 20);
        data.insert("key3".to_string(), 30);
        let dict = data.clone().into_py_dict(py);
        Ok(dict.to_object(py))
    }

    #[getter]
    fn stats(&self) -> PyResult<String> {
        match serde_json::to_string(&create_stats(&self)) {
            Ok(json_string) => Ok(json_string),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(err.to_string())),
        }
    }
}
