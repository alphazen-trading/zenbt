use std::fmt::Display;
use chrono::{DateTime, Utc, TimeZone};
use pyo3::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::prelude::FromPrimitive;
use super::ohlc::OHLC;
use std::ops::Div;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Backtest {
    pub ohlc: Vec<OHLC>,
}
#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Backtest {
    #[new]
    fn new(
        data: Vec<Vec<f64>>,
    ) -> Self {
        let mut ohlcs = Vec::new();

        for i in 0..data.len() {
            let ohlc = OHLC {
                date: DateTime::from_timestamp(data[i][0].to_i64().unwrap().div(1000), 0).expect("Invalid timestamp"),
                open: Decimal::from_f64(data[i][1]).unwrap(),
                high: Decimal::from_f64(data[i][2]).unwrap(),
                low: Decimal::from_f64(data[i][3]).unwrap(),
                close: Decimal::from_f64(data[i][4]).unwrap(),
                volume: Decimal::from_f64(data[i][5]).unwrap(),
            };
            ohlcs.push(ohlc);
        }
        Backtest {
            ohlc: ohlcs
        } 
    }
    #[getter]
    fn print(&self) -> PyResult<Decimal> {
        println!("{:?}", self);
        Ok(self.ohlc[0].open)
    }
}
