use super::ohlc::OHLC;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Serialize;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone, Serialize)]
pub struct Position {
    #[serde(with = "rust_decimal::serde::float")]
    pub index: Decimal,
    pub entry_timestamp: DateTime<Utc>,
    pub exit_timestamp: Option<DateTime<Utc>>,
    pub entry_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub size: Decimal,
    pub sl: Decimal,
    pub tp: Decimal,
    pub side: Decimal,
    pub pnl: Decimal,
    pub max_dd: Decimal,
    pub close_reason: Option<String>,
    pub commission: Decimal,
    pub commission_pct: Decimal,
}
impl ToPyObject for Position {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new_bound(py);

        // Convert and insert fields
        dict.set_item("index", self.index).unwrap();
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

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Position {
    pub fn update_pnl(&mut self, close: Decimal) {
        if self.side == dec!(1.0) {
            self.pnl = (close - self.entry_price) * self.size;
        } else {
            self.pnl = (self.entry_price - close) * self.size;
        }
        if self.pnl < self.max_dd {
            self.max_dd = self.pnl
        }
    }
    pub fn was_sl_hit(&mut self, ohlc: &OHLC) -> bool {
        if self.side == dec!(1.0) {
            if ohlc.low <= self.sl {
                self.exit_timestamp = Some(ohlc.date);
                self.exit_price = Some(self.sl);
                self.close_reason = Some(String::from("stop_loss"));
                self.commission += self.commission_pct * self.exit_price.unwrap() * self.size;
                self.pnl =
                    (self.exit_price.unwrap() - self.entry_price) * self.size - self.commission;
                return true;
            }
        } else {
            if ohlc.low >= self.sl {
                self.exit_timestamp = Some(ohlc.date);
                self.exit_price = Some(self.sl);
                self.close_reason = Some(String::from("stop_loss"));
                self.commission += self.commission_pct * self.exit_price.unwrap() * self.size;
                self.pnl =
                    (self.entry_price - self.exit_price.unwrap()) * self.size - self.commission;
                return true;
            }
        }
        return false;
    }
    pub fn was_tp_hit(&mut self, ohlc: &OHLC) -> bool {
        if self.side == dec!(1.0) {
            if ohlc.high >= self.tp {
                self.exit_timestamp = Some(ohlc.date);
                self.exit_price = Some(self.tp);
                self.close_reason = Some(String::from("take_profit"));
                self.commission += self.commission_pct * self.exit_price.unwrap() * self.size;
                self.pnl =
                    (self.exit_price.unwrap() - self.entry_price) * self.size - self.commission;
                return true;
            }
        } else {
            if ohlc.high <= self.tp {
                self.exit_timestamp = Some(ohlc.date);
                self.exit_price = Some(self.tp);
                self.close_reason = Some(String::from("take_profit"));
                self.commission += self.commission_pct * self.exit_price.unwrap() * self.size;
                self.pnl =
                    (self.entry_price - self.exit_price.unwrap()) * self.size - self.commission;
                return true;
            }
        }
        return false;
    }
    pub fn should_close(&mut self, ohlc: &OHLC) -> bool {
        return self.was_sl_hit(ohlc) || self.was_tp_hit(ohlc);
    }
}

#[derive(Debug, Clone, Serialize)]
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
