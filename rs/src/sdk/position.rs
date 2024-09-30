use super::ohlc::OHLC;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::Decimal;

use super::enums::{CloseReason, Side};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub exit_index: usize,
    pub entry_timestamp: DateTime<Utc>,
    pub exit_timestamp: Option<DateTime<Utc>>,
    pub entry_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub size: Decimal,
    pub sl: Option<Decimal>,
    pub tp: Option<Decimal>,
    pub side: Side,
    pub pnl: Decimal,
    pub max_dd: Decimal,
    pub close_reason: Option<CloseReason>,
    pub commission: Decimal,
    pub commission_pct: Decimal,
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

#[pymethods]
impl Position {
    pub fn update_pnl(&mut self, close: Decimal) {
        if self.side == Side::Long {
            self.pnl = (close - self.entry_price) * self.size;
        } else {
            self.pnl = (self.entry_price - close) * self.size;
        }
        if self.pnl < self.max_dd {
            self.max_dd = self.pnl
        }
    }
    pub fn close_position(
        &mut self,
        i: usize,
        ohlc: &OHLC,
        exit_price: Decimal,
        close_reason: CloseReason,
        pnl: Decimal,
    ) {
        self.exit_timestamp = Some(ohlc.date);
        self.exit_index = i;
        self.exit_price = Some(exit_price);
        self.close_reason = Some(close_reason);
        self.commission += self.commission_pct * self.exit_price.unwrap() * self.size;
        self.pnl = pnl;
    }

    pub fn was_sl_hit(&mut self, i: usize, ohlc: &OHLC) -> bool {
        if let Some(sl_price) = self.sl {
            if self.side == Side::Long {
                if ohlc.low <= sl_price {
                    let pnl = (sl_price - self.entry_price) * self.size - self.commission;
                    self.close_position(i, ohlc, sl_price, CloseReason::StopLoss, pnl);
                    return true;
                }
            } else {
                if ohlc.high >= sl_price {
                    let pnl = (self.entry_price - sl_price) * self.size - self.commission;
                    self.close_position(i, ohlc, sl_price, CloseReason::StopLoss, pnl);
                    return true;
                }
            }
        }
        false
    }
    pub fn was_tp_hit(&mut self, i: usize, ohlc: &OHLC) -> bool {
        if let Some(tp_price) = self.tp {
            if self.side == Side::Long {
                if ohlc.high >= tp_price {
                    let pnl = (tp_price - self.entry_price) * self.size - self.commission;
                    self.close_position(i, ohlc, tp_price, CloseReason::TakeProfit, pnl);
                    return true;
                }
            } else {
                if ohlc.low <= tp_price {
                    let pnl = (self.entry_price - tp_price) * self.size - self.commission;
                    self.close_position(i, ohlc, tp_price, CloseReason::TakeProfit, pnl);
                    return true;
                }
            }
        }
        false
    }
    pub fn should_close(&mut self, i: usize, ohlc: &OHLC) -> bool {
        return self.was_sl_hit(i, ohlc) || self.was_tp_hit(i, ohlc);
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
