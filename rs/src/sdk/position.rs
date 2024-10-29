use super::enums::{CloseReason, Side};
use crate::backtest::helpers::{get_date_at_index, get_value_at};
use crate::backtest::params::BacktestParams;
use crate::sdk::order::Order;
use crate::strategy::actions::Action;
use chrono::{DateTime, Utc};
use polars::frame::DataFrame;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;

#[pyclass()]
#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub id: String,
    pub index: usize,
    pub exit_index: usize,
    pub entry_timestamp: DateTime<Utc>,
    pub exit_timestamp: Option<DateTime<Utc>>,
    pub entry_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub size: Decimal,
    pub sl: Option<Decimal>,
    pub tp: Option<Decimal>,
    #[pyo3(get)]
    pub side: Side,
    pub pnl: Decimal,
    pub max_dd: Decimal,
    pub close_reason: Option<CloseReason>,
    pub commission: Decimal,
    pub commission_pct: Decimal,
}
impl Default for Positions {
    fn default() -> Self {
        Self::new()
    }
}

impl ToPyObject for Position {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new_bound(py);

        // Convert and insert fields
        dict.set_item("index", self.index).unwrap();
        dict.set_item("exit index", self.exit_index).unwrap();
        dict.set_item("entry_timestamp", self.entry_timestamp.to_rfc3339())
            .unwrap(); // DateTime<Utc> as string

        // Option handling for exit_timestamp
        if let Some(exit_ts) = self.exit_timestamp {
            dict.set_item("exit_timestamp", exit_ts.to_rfc3339())
                .unwrap();
        } else {
            dict.set_item("exit_timestamp", py.None()).unwrap();
        }

        dict.set_item("entry_price", self.entry_price).unwrap();

        // Option handling for exit_price
        if let Some(exit_price) = self.exit_price {
            dict.set_item("exit_price", exit_price).unwrap();
        } else {
            dict.set_item("exit_price", py.None()).unwrap();
        }

        dict.set_item("size", self.size).unwrap();
        dict.set_item("sl", self.sl).unwrap();
        dict.set_item("tp", self.tp).unwrap();
        dict.set_item("side", self.side).unwrap();
        dict.set_item("pnl", self.pnl).unwrap();
        dict.set_item("max_dd", self.max_dd).unwrap();

        // Option handling for close_reason
        if let Some(reason) = &self.close_reason {
            dict.set_item("close_reason", reason).unwrap();
        } else {
            dict.set_item("close_reason", py.None()).unwrap();
        }

        dict.set_item("commission", self.commission).unwrap();
        dict.set_item("commission_pct", self.commission_pct)
            .unwrap();

        dict.to_object(py)
    }
}

impl Position {
    pub fn update_pnl(&mut self, i: usize, df: &DataFrame) {
        let close = get_value_at(df, i, "close");
        if self.side == Side::Long {
            self.pnl = (close - self.entry_price) * self.size;
        } else {
            self.pnl = (self.entry_price - close) * self.size;
        }
        if self.pnl < self.max_dd {
            self.max_dd = self.pnl;
        }
    }
    pub fn close_position(
        &mut self,
        i: usize,
        df: &DataFrame,
        exit_price: Decimal,
        close_reason: CloseReason,
        // pnl: Decimal,
    ) {
        let date = get_date_at_index(df, i);
        self.exit_timestamp = Some(date);
        self.exit_index = i;
        self.exit_price = Some(exit_price);
        self.close_reason = Some(close_reason);
        self.commission += self.commission_pct * self.exit_price.unwrap() * self.size;
        self.update_pnl(i, df);
    }

    pub fn was_sl_hit(&mut self, i: usize, df: &DataFrame) -> bool {
        if let Some(sl_price) = self.sl {
            if self.side == Side::Long {
                let low = get_value_at(df, i, "low");
                if low <= sl_price {
                    self.pnl = (sl_price - self.entry_price) * self.size - self.commission;
                    self.close_position(i, df, sl_price, CloseReason::StopLoss);
                    return true;
                }
            } else {
                let high = get_value_at(df, i, "high");
                if high >= sl_price {
                    self.pnl = (self.entry_price - sl_price) * self.size - self.commission;
                    self.close_position(i, df, sl_price, CloseReason::StopLoss);
                    return true;
                }
            }
        }
        false
    }
    pub fn was_tp_hit(&mut self, i: usize, df: &DataFrame) -> bool {
        if let Some(tp_price) = self.tp {
            if self.side == Side::Long {
                let high = get_value_at(df, i, "high");
                if high >= tp_price {
                    self.pnl = (tp_price - self.entry_price) * self.size - self.commission;
                    self.close_position(i, df, tp_price, CloseReason::TakeProfit);
                    return true;
                }
            } else {
                let low = get_value_at(df, i, "low");
                if low <= tp_price {
                    self.pnl = (self.entry_price - tp_price) * self.size - self.commission;
                    self.close_position(i, df, tp_price, CloseReason::TakeProfit);
                    return true;
                }
            }
        }
        false
    }
    pub fn is_position_no_longer_desired(
        &mut self,
        i: usize,
        df: &DataFrame,
        action: &Action,
    ) -> bool {
        if !action.positions.contains_key(&self.id) {
            let open = get_value_at(df, i + 1, "open");
            if self.side == Side::Long {
                self.pnl = (open - self.entry_price) * self.size - self.commission;
            } else {
                self.pnl = (self.entry_price - open) * self.size - self.commission;
            }
            self.close_position(i, df, open, CloseReason::Manual);
            return true;
        }
        false
    }
    pub fn should_close(&mut self, i: usize, df: &DataFrame, action: &Action) -> bool {
        self.was_sl_hit(i, df)
            || self.was_tp_hit(i, df)
            || self.is_position_no_longer_desired(i, df, action)
    }

    fn __repr__(&self) -> String {
        // Serialize the struct to a JSON string using serde_json
        match serde_json::to_string(self) {
            Ok(json_string) => json_string,
            Err(_) => "Failed to serialize Order struct".to_string(),
        }
    }

    pub fn create_position(
        order: &Order,
        date: DateTime<Utc>,
        params: &BacktestParams,
    ) -> Position {
        let entry_price = order.price.expect("Order price is None!");

        Position {
            id: rand::thread_rng().gen_range(0..999_999_999).to_string(),
            index: order.index,
            exit_index: 0,
            entry_timestamp: date,
            exit_timestamp: None,
            entry_price,
            exit_price: None,
            size: order.size,
            sl: order.sl,
            tp: order.tp,
            side: order.side,
            close_reason: None,
            pnl: dec!(0.0),
            max_dd: dec!(0.0),
            commission: entry_price * params.commission_pct * order.size,
            commission_pct: params.commission_pct,
        }
    }
}

#[pyclass()]
#[derive(Debug, Clone)]
pub struct Positions {
    pub active_positions: Vec<Position>,
    pub closed_positions: Vec<Position>,
}

impl Positions {
    pub fn new() -> Self {
        Positions {
            active_positions: Vec::new(),
            closed_positions: Vec::new(),
        }
    }
}
