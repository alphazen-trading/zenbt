use pyo3::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json::Value;

#[cfg_attr(feature = "pyi", pyi_macros::pyi)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Contract {
    pub id: u64,
    pub code: String,
    pub tick_size: Decimal,
    pub min_order: Decimal,
    pub min_value: Decimal,
    pub last_price: Decimal,
    pub active: bool,
    pub min_order_usd_value: Decimal,
    pub exchange_base_underlying: String,
    pub exchange_counter_underlying: String,
    pub underlyings_codes: Vec<String>,
    value: Value,
}

#[cfg_attr(feature = "pyi", pyi_macros::pyi_impl)]
#[pymethods]
impl Contract {
    #[new]
    fn new(_json: String) -> Self {
        let v: Value = serde_json::from_str(_json.to_string().as_str()).unwrap();
        let exchange_base_underlying: String = v["exchange_base_underlying"]
            .to_string()
            .trim_matches('"')
            .to_string();
        let exchange_counter_underlying: String = v["exchange_counter_underlying"]
            .to_string()
            .trim_matches('"')
            .to_string();

        Contract {
            id: v["id"].as_u64().unwrap(),
            code: v["code"].to_string(),
            tick_size: serde_json::from_value(v["ticksize"].clone()).unwrap_or_default(),
            min_order: serde_json::from_value(v["min_order"].clone()).unwrap_or_default(),
            min_value: serde_json::from_value(v["min_value"].clone()).unwrap_or_default(),
            min_order_usd_value: serde_json::from_value(v["min_value"].clone()).unwrap_or_default(),
            last_price: serde_json::from_value(v["last_price"].clone()).unwrap_or_default(),
            active: v["active"].as_bool().unwrap(),
            exchange_base_underlying: exchange_base_underlying.clone(),
            exchange_counter_underlying: exchange_counter_underlying.clone(),
            underlyings_codes: vec![exchange_base_underlying, exchange_counter_underlying],
            value: v,
        }
    }

    #[getter]
    fn id(&self) -> u64 {
        self.id
    }

    #[getter]
    fn code(&self) -> String {
        self.code.clone()
    }

    #[getter]
    fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    #[getter]
    fn min_order(&self) -> Decimal {
        self.min_order
    }

    #[getter]
    fn min_order_usd_value(&self) -> Decimal {
        self.min_order_usd_value
    }

    #[getter]
    fn last_price(&self) -> Decimal {
        self.last_price
    }

    #[getter]
    pub fn is_active(&self) -> bool {
        self.active
    }

    #[getter]
    fn exchange_base_underlying(&self) -> String {
        self.exchange_base_underlying.clone()
    }

    #[getter]
    fn exchange_counter_underlying(&self) -> String {
        self.exchange_counter_underlying.clone()
    }

    // #[getter]
    // fn value(&self) -> Value {
    //     self.value.clone()
    // }

    fn print(&self) -> () {
        println!("{}", self.value);
    }
}
