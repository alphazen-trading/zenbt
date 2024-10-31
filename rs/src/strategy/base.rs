use std::collections::HashMap;

use numpy::ToPyArray;
use polars::prelude::*;
use rust_decimal_macros::dec;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use pyo3_polars::PyDataFrame;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

use crate::backtest::helpers::get_value_at;
use crate::sdk::enums::{OrderType, Side};
use crate::sdk::order::Order;

use super::actions::Action;

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Strategy {
    pub df: PyDataFrame,
    pub data: Py<PyDict>,
    pub equity: Vec<Decimal>,
    pub default_size: Decimal,
    pub action: Action,
    pub index: isize,
}

#[pymethods]
impl Strategy {
    #[new]
    fn new(df: PyDataFrame, default_size: Decimal) -> PyResult<Strategy> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            let col_names = df.0.get_column_names();
            for (i, col) in df.0.get_columns().iter().enumerate() {
                if col.dtype() == &DataType::Float64 {
                    let col_values: Vec<f64> = col
                        .f64()
                        .unwrap()
                        .into_iter()
                        .collect::<Option<Vec<f64>>>()
                        .unwrap();
                    dict.set_item(col_names[i].to_string(), col_values.to_pyarray_bound(py))?;
                } else if col.dtype() == &DataType::Int64 {
                    let col_values: Vec<i64> = col
                        .i64()
                        .unwrap()
                        .into_iter()
                        .collect::<Option<Vec<i64>>>()
                        .unwrap();
                    dict.set_item(col_names[i].to_string(), col_values.to_pyarray_bound(py))?;
                } else if col.dtype() == &DataType::Boolean {
                    let col_values: Vec<bool> = col
                        .bool()
                        .unwrap()
                        .into_iter()
                        .collect::<Option<Vec<bool>>>()
                        .unwrap();
                    dict.set_item(col_names[i].to_string(), col_values.to_pyarray_bound(py))?;
                } else {
                    // Handle unsupported data types or skip
                    eprintln!(
                        "Unsupported column type for column: {}, {:?}",
                        col_names[i],
                        col.dtype()
                    );
                    continue;
                }
            }

            Ok(Strategy {
                df,
                data: dict.into(),
                equity: Vec::new(),
                default_size,
                action: Action {
                    orders: HashMap::new(),
                    close_all_positions: false,
                },
                index: -1,
            })
        })
    }

    #[classmethod]
    #[allow(unused_variables)]
    pub fn _on_candle(cls: &Bound<'_, PyType>) -> i32 {
        10
    }
    #[classmethod]
    #[allow(unused_variables)]
    pub fn on_candle(cls: &Bound<'_, PyType>) -> i32 {
        10
    }

    #[pyo3(signature = (index, client_order_id, side, size, price, sl=None, tp=None))]
    #[allow(clippy::similar_names, clippy::too_many_arguments)]
    pub fn create_limit_order(
        &self,
        index: usize,
        client_order_id: String,
        side: Side,
        size: Decimal,
        price: f64,
        sl: Option<Decimal>,
        tp: Option<Decimal>,
    ) -> Order {
        let _ = self;
        let price = Decimal::from_f64(price)
            .ok_or(
                "The price passed for the new limit order is not a valid float. (Maybe it's NaN?)",
            )
            .unwrap();
        Order {
            index,
            client_order_id,
            order_type: OrderType::Limit,
            side,
            size,
            price: Some(price),
            sl,
            tp,
        }
    }

    fn update_index(&mut self) {
        self.index += 1;
    }

    fn reset_action(&mut self) {
        self.action.reset();
    }

    fn add_order(&mut self, order: Order) {
        self.action
            .orders
            .insert(order.client_order_id.clone(), order);
    }

    #[pyo3(signature = (index, client_order_id, side, size, sl=None, tp=None))]
    #[allow(clippy::similar_names)]
    pub fn create_market_order(
        &self,
        index: usize,
        client_order_id: String,
        side: Side,
        size: Decimal,
        sl: Option<Decimal>,
        tp: Option<Decimal>,
    ) -> Order {
        let _ = self;
        Order {
            index,
            client_order_id,
            order_type: OrderType::Market,
            side,
            size,
            price: None,
            sl,
            tp,
        }
    }
}
impl Strategy {
    #[allow(clippy::similar_names)]
    pub fn rs_create_market_order(
        &self,
        index: usize,
        client_order_id: String,
        side: Side,
        size: Decimal,
        sl: Option<Decimal>,
        tp: Option<Decimal>,
    ) -> Order {
        let _ = self;
        Order {
            index,
            client_order_id,
            order_type: OrderType::Market,
            side,
            size,
            price: None,
            sl,
            tp,
        }
    }
    pub fn fast_method_test(&self, i: usize, df: &DataFrame) -> Action {
        let cross_below = get_value_at(df, i, "cross_below");
        let cross_above = get_value_at(df, i, "cross_above");
        let mut orders = HashMap::new();
        let mut close_all_positions = false;
        if cross_above == dec!(1) {
            let order = self.rs_create_market_order(
                i,
                "Long".to_string(),
                Side::Long,
                self.default_size,
                None,
                None,
            );
            orders.insert(order.client_order_id.clone(), order);
            close_all_positions = true;
        }
        if cross_below == dec!(1) {
            let order = self.create_market_order(
                i,
                "Long".to_string(),
                Side::Long,
                self.default_size,
                None,
                None,
            );
            orders.insert(order.client_order_id.clone(), order);
            close_all_positions = true;
        }
        Action {
            orders,
            close_all_positions,
        }
    }
}
