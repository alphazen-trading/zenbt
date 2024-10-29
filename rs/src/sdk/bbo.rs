use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass]
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct BBO {
    pub time: DateTime<Utc>,
    pub code: String,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    pub bid_size: Decimal,
    pub ask_size: Decimal,
}

#[pymethods]
impl BBO {
    #[new]
    fn new(
        time: Decimal,
        code: String,
        bid_size: Decimal,
        bid_price: Decimal,
        ask_size: Decimal,
        ask_price: Decimal,
    ) -> Self {
        BBO {
            time: DateTime::from_timestamp(time.to_i64().unwrap(), 0).expect("Invalid timestamp"),
            code,
            bid_price,
            ask_price,
            bid_size,
            ask_size,
        }
    }

    #[getter]
    pub fn print(&self) {
        println!(
            "{} --- ({}) BID: {} @ {} - ASK: {} @ {}",
            self.time.timestamp(),
            self.code,
            self.bid_size,
            self.bid_price,
            self.ask_size,
            self.ask_price
        );
    }
}

impl Default for BBO {
    fn default() -> Self {
        Self {
            time: Utc::now(),
            code: String::new(),
            bid_price: dec!(0.0),
            ask_price: dec!(0.0),
            bid_size: dec!(0.0),
            ask_size: dec!(0.0),
        }
    }
}
