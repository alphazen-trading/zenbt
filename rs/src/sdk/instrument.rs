use super::bbo::BBO;
use super::contract::Contract;
use crate::helpers::round_value::round_value;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Instrument {
    pub contract: Contract,
    pub bbo: BBO,
    pub min_order_quantity: Decimal,
    pub min_order: Decimal,
    pub min_order_usd_value: Decimal,
    pub last_timestamp: DateTime<Utc>,
    pub underlyings_codes: Vec<String>,
    pub code: String,
    pub id: u64,
}

#[pymethods]
impl Instrument {
    #[new]
    fn new(contract: Contract) -> Self {
        let mut instrument = Self {
            contract: contract.clone(),
            bbo: BBO::default(),
            min_order_quantity: dec!(0.0),
            min_order: contract.min_order,
            min_order_usd_value: contract.min_value,
            last_timestamp: Utc::now(),
            underlyings_codes: contract.underlyings_codes.clone(),
            code: contract.code.replace('"', ""),
            id: contract.id,
        };
        if contract.is_active() {
            instrument.min_order_quantity = round_value(
                contract.min_order_usd_value / contract.last_price,
                contract.min_order,
            ) + contract.min_order;
        }

        instrument
    }

    #[getter]
    fn min_order_quantity(&self) -> PyResult<Decimal> {
        Ok(self.min_order_quantity)
    }

    #[getter]
    fn contract(&self) -> PyResult<Contract> {
        Ok(self.contract.clone())
    }

    #[getter]
    fn bbo(&self) -> PyResult<BBO> {
        Ok(self.bbo.clone())
    }

    #[getter]
    fn underlyings_codes(&self) -> PyResult<Vec<String>> {
        Ok(self.underlyings_codes.clone())
    }

    #[getter]
    fn code(&self) -> PyResult<String> {
        Ok(self.code.clone())
    }

    #[getter]
    fn id(&self) -> PyResult<u64> {
        Ok(self.id)
    }

    pub fn on_new_bbo(&mut self, bbo: BBO) {
        self.last_timestamp = bbo.time;
        self.bbo = bbo;

        self.min_order_quantity = round_value(
            self.contract.min_order_usd_value / self.bbo.ask_price,
            self.contract.min_order,
        ) + self.contract.min_order;
    }

    fn round_price(&self, value: Decimal) -> PyResult<Decimal> {
        Ok(round_value(value, self.contract.tick_size))
    }

    pub fn round_amount(&self, value: Decimal) -> PyResult<Decimal> {
        Ok(round_value(value, self.contract.min_order))
    }

    fn is_active(&self) -> PyResult<bool> {
        Ok(self.contract.active)
    }
}
