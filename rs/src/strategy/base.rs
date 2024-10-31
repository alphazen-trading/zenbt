use std::collections::HashMap;

use numpy::ToPyArray;
use polars::prelude::*;
use rust_decimal_macros::dec;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use pyo3_polars::PyDataFrame;
use rust_decimal::Decimal;

use crate::backtest::helpers::get_value_at;
use crate::sdk::enums::{OrderType, Side};
use crate::sdk::order::Order;

use super::actions::Action;

#[pyclass(get_all, subclass, frozen)]
#[derive(Debug)]
pub struct Strategy {
    pub df: PyDataFrame,
    pub data: Py<PyDict>,
    pub equity: Vec<Decimal>,
    pub default_size: Decimal,
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

    // #[classmethod]
    // fn override_test(_cls: &Bound<'_, PyType>) -> PyResult<i32> {
    //     println!("print within rust");
    //     Ok(10)
    // }

    // fn backtest(slf: &Bound<Self>) -> PyResult<i32> {
    //     println!("print from rust: {}", 23);
    //     let action: Action = slf
    //         .call_method("override_test", (), None)
    //         .unwrap()
    //         .extract()
    //         .unwrap();
    //     println!("The cls is: {:?}", action);
    //     println!("The cls is: {:?}", action.desired_orders);
    //     // let action: Action = slf.getattr("desired_action").unwrap().extract().unwrap();
    //     // println!("The value of it is: {:?}", action);
    //     // println!("The value of it is: {:?}", action.desired_orders);
    //     Ok(32)
    // }
    // fn backtest(&mut self, py: Python<'_>, wraps: Py<PyAny>) {
    //     let df = self.df.0.clone();
    //     // let res = slf.call_method_bound(py, intern!(py, "_on_candle"), (0, self), None);
    //     let res: Action = wraps
    //         .call_bound(py, (0, self.state.clone()), None)
    //         .unwrap()
    //         .extract(py)
    //         .unwrap();
    //     println!("The result is: {:?}", res);
    //     for i in 0..df.height() {
    //         //     // for (j, position) in &mut backtest.positions.active_positions.iter_mut().enumerate() {
    //         //     // for pos in &mut self.active_positions {
    //         //     //     println!("{:?}", pos);
    //         //     // }
    //         //
    //         // let res = slf.call_method_bound(py, intern!(py, "_on_candle"), (i,), None);
    //         // let action = res;

    //         // // let action: Action = slf
    //         // //     .call_method_bound(py, intern!(py, "_on_candle"), (i,), None)
    //         // //     .unwrap()
    //         // //     .extract(py)
    //         // //     .unwrap();
    //         // println!("The result is: {:?}", action);
    //     }
    // }
    // fn foo(slf: &Bound<Self>) -> PyResult<()> {
    //     slf.call_method0("override_test")?;
    //     Ok(())
    // }
    // fn backtest(&self, py: Python<'_>) {
    //     Self::override_test(self.into_py(py));
    // }

    // fn backtest(&self, py: Python<'_>) {
    //     let cls = py.get_type_bound::<Strategy>();
    //     let df = cls.getattr("desired_action");
    //     let result = cls
    //         .call_method(intern!(py, "override_test"), (), None)
    //         .unwrap();
    //     //     // println!("The cls is: {:?}", cls);
    //     println!("The cls is: {:?}", df);
    // }

    //     let result = cls
    //         .call_method(intern!(py, "override_test"), (), None)
    //         .unwrap();
    //     println!("The cls is: {:?}", result);
    //     // .unwrap()
    //     // .extract(py)
    //     // .unwrap();
    //     // cls.call_method(py, "override_test", (self), None).unwrap();
    //     // self.into_py(py)
    //     //     .call_method_bound(py, "_on_candle", (), None)
    //     //     .unwrap();
    //     // let ret = self.wraps.call_bound(py, (self.into_py(py)), None).unwrap();
    //     // println!("{:?}", ret);
    //     // self.wraps.call_method_bound(
    //     //     py,
    //     //     // intern!(py, "_on_candle"),
    //     //     "_on_candle",
    //     //     (0, self),
    //     //     None,
    //     // )
    //     // let cls = &Self::override_test(self.borrow

    //     // py.allow_threads(move || {
    //     //     // An example of an "expensive" Rust calculation
    //     //     // let sum = numbers.iter().sum();

    //     //     Ok(1)
    //     // })
    //     // Python::with_gil(|py| {
    //     //     Self::
    //     //     // let res = Self::test(&Self::type_object_bound(py));
    //     //     // println!("The res is: {}", res);
    //     //     // Self::call_method_bound(py, intern!(py, "test"), (), None).unwrap();

    //     //     let result: Action = Self::test(&Self::type_object_bound(py));
    //     //     println!("The result is: {:?}", result);
    //     //     // .strategy
    //     //     // .call_method_bound(
    //     //     //     py,
    //     //     //     intern!(py, "_on_candle"),
    //     //     //     (i, self.state.borrow(py), test.clone()),
    //     //     //     None,
    //     //     // )
    //     //     // .unwrap()
    //     //     // .extract(py)
    //     //     // .unwrap();
    //     //     // result.extract::<Action>(py).unwrap()
    //     // })
    // }

    // #[classmethod]
    // fn backtest(cls: &Bound<'_, PyType>) {
    //     let result = cls.call_method("override_test", (cls,), None).unwrap();
    //     let action = cls.getattr("desired_action").unwrap();
    //     println!("The result is: {:?}", action);
    // }
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
            position: None,
            close_all_positions,
        }
    }
}
