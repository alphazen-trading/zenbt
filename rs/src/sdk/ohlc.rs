use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct OHLC {
    pub date: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
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
        let date = DateTime::from_timestamp(timestamp.to_i64().unwrap(), 0).expect("Invalid timestamp");
        OHLC {
            date,
            open,
            high,
            low,
            close,
            volume,
        }
    }
    #[getter]
    fn print(&self) -> PyResult<Decimal> {
        println!("{:?}", self);
        Ok(self.open)
    }
}
