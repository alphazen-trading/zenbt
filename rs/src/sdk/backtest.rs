use super::backtest_methods::{has_account_blown_up, was_order_hit};
use super::ohlc::OHLC;
use super::order::Order;
use super::position::Position;
use super::stats::{calculate_max_drawdown, Stats};
use chrono::{DateTime, TimeZone, Utc};
use pyo3::prelude::*;
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
    pub commission_pct: Decimal,
    pub commissions: Decimal,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub initial_capital: Decimal,
}
#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Backtest {
    #[new]
    fn new(data: Vec<Vec<f64>>, commission_pct: Decimal, initial_capital: Decimal) -> Self {
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
            commission_pct,
            commissions: dec![0],
            equity: Vec::new(),
            floating_equity: Vec::new(),
            initial_capital,
        }
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
        self.trailing_tp = Vec::new();
        self.active_positions = Vec::new();
        self.closed_positions = Vec::new();
        self.commissions = dec![0];
        self.equity = Vec::new();
        self.floating_equity = Vec::new();
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
                .push(self.equity.last().unwrap_or(&self.initial_capital) + realized_equity);

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
                            let mut new_position = Position {
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
                                commission: order.price * self.commission_pct * order.size,
                                commission_pct: self.commission_pct,
                            };
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

    #[getter]
    fn stats(&self) -> PyResult<String> {
        let mut wins = dec!(0);
        let mut losses = dec!(0);
        for position in self.closed_positions.clone() {
            if position.pnl > dec!(0.0) {
                wins += dec!(1);
            } else {
                losses += dec!(1);
            }
        }
        let mut commissions = self.commissions;
        for position in &self.active_positions {
            commissions += position.commission;
        }

        let mut win_rate = dec!(0);
        if wins + losses > dec!(0) {
            win_rate = (wins / (wins + losses) * dec!(100.0)).round_dp(2);
        }

        let max_drawdown = calculate_max_drawdown(&self.equity).unwrap_or(dec!(0.0));
        let pnl = self.equity.last().unwrap() - self.initial_capital
            + self.floating_equity.last().unwrap();
        let stats = Stats {
            initial_capital: self.initial_capital,
            pnl,
            pnl_pct: pnl * dec!(100) / self.initial_capital,
            unrealized_pnl: *self.floating_equity.last().unwrap(),
            total_positions: self.active_positions.len() + self.closed_positions.len(),
            closed_positions: self.closed_positions.len(),
            active_positions: self.active_positions.len(),
            commissions,
            wins,
            losses,
            win_rate,
            trading_days: self
                .ohlc
                .last()
                .unwrap()
                .date
                .signed_duration_since(self.ohlc.first().unwrap().date)
                .num_days(),
            start_date: self.ohlc.first().unwrap().date.to_string(),
            end_date: self.ohlc.last().unwrap().date.to_string(),
            max_drawdown,
            max_drawdown_pct: max_drawdown * dec!(100) / self.initial_capital,
        };
        match serde_json::to_string(&stats) {
            Ok(json_string) => Ok(json_string),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(err.to_string())),
        }
    }
}
