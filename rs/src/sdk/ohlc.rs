use chrono::{DateTime, Utc};
use pyo3::prelude::*;
// use pyo3::types::PyDict;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::ops::Div;

use serde::Serialize;

// A representation of a time based Bar
// TODO Need to remove this
#[pyclass]
#[derive(Debug, Clone, Serialize, Copy)]
pub struct OHLC {
    pub date: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Copy)]
pub struct OHLCV {
    pub date: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

// Where we create the Actual OHLCs

#[pyclass]
#[derive(Debug, Clone, Serialize)]
pub struct OHLCs {
    pub ohlc: Vec<OHLC>,
}

#[pymethods]
impl OHLCs {
    #[new]
    fn new(data: Vec<Vec<f64>>) -> Self {
        let mut ohlcs = Vec::new();

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
        OHLCs { ohlc: ohlcs }
    }
}
