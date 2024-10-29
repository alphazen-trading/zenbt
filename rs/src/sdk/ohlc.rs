use chrono::{DateTime, Utc};
use pyo3::prelude::*;
// use pyo3::types::PyDict;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::ops::Div;

use serde::Serialize;

// A representation of a time based Bar
#[pyclass]
#[derive(Debug, Clone, Serialize, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub struct OHLC {
    pub date: DateTime<Utc>,
    #[pyo3(get)]
    pub open: Decimal,
    #[pyo3(get)]
    pub high: Decimal,
    #[pyo3(get)]
    pub low: Decimal,
    #[pyo3(get)]
    pub close: Decimal,
    #[pyo3(get)]
    pub volume: Decimal,
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Copy)]
#[allow(clippy::upper_case_acronyms)]
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
    #[allow(clippy::needless_pass_by_value)]
    fn new(data: Vec<Vec<f64>>) -> Self {
        let mut ohlcs = Vec::new();

        for row in &data {
            let ohlc = OHLC {
                date: DateTime::from_timestamp(row[0].to_i64().unwrap().div(1000), 0)
                    .expect("Invalid timestamp"),
                open: Decimal::from_f64(row[1]).unwrap_or(Decimal::from(0)),
                high: Decimal::from_f64(row[2]).unwrap_or(Decimal::from(0)),
                low: Decimal::from_f64(row[3]).unwrap_or(Decimal::from(0)),
                close: Decimal::from_f64(row[4]).unwrap_or(Decimal::from(0)),
                volume: Decimal::from_f64(row[5]).unwrap_or(Decimal::from(0)),
            };
            ohlcs.push(ohlc);
        }
        OHLCs { ohlc: ohlcs }
    }
}
