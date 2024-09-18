use super::ohlc::OHLC;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
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

    pub fn print(&self) {
        println!("{:?}", self);
    }

    #[getter]
    fn entry_price(&self) -> PyResult<Decimal> {
        Ok(self.entry_price)
    }

    #[getter]
    fn exit_price(&self) -> PyResult<Decimal> {
        Ok(self.exit_price.unwrap())
    }

    #[getter]
    fn pnl(&self) -> PyResult<Decimal> {
        Ok(self.pnl)
    }

    #[getter]
    fn size(&self) -> PyResult<Decimal> {
        Ok(self.size)
    }

    #[getter]
    fn close_reason(&self) -> PyResult<String> {
        Ok(self.close_reason.clone().unwrap())
    }

    #[getter]
    fn max_dd(&self) -> PyResult<Decimal> {
        Ok(self.max_dd)
    }

    #[getter]
    fn commission(&self) -> PyResult<Decimal> {
        Ok(self.commission)
    }

    pub fn to_json(&self) -> PyResult<String> {
        match serde_json::to_string(&self) {
            Ok(json_string) => Ok(json_string),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(err.to_string())),
        }
    }
}
