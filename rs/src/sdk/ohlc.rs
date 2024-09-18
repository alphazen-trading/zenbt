use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use serde::Serialize;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
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
impl ToPyObject for OHLC {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new_bound(py);
        dict.set_item("date", self.date.to_rfc3339()).unwrap();
        dict.set_item("open", self.open).unwrap();
        dict.set_item("high", self.high).unwrap();
        dict.set_item("low", self.low).unwrap();
        dict.set_item("close", self.close).unwrap();
        dict.set_item("volume", self.volume).unwrap();
        dict.to_object(py)
    }
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl OHLC {
    #[new]
    fn new(
        timestamp: Decimal,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        let date =
            DateTime::from_timestamp(timestamp.to_i64().unwrap(), 0).expect("Invalid timestamp");
        OHLC {
            date,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
