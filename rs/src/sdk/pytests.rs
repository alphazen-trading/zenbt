use polars_core::frame::DataFrame;
use pyo3::{prelude::*, types::PyType};

// use super::ohlc::OHLC;
use chrono::{DateTime, Utc};
use pyo3_polars::PyDataFrame;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[pyclass(get_all, set_all)]
#[derive(Debug)]
pub struct Test {
    pub mine: Decimal,
}

#[pyclass(get_all, frozen)]
#[derive(Debug)]
pub struct Foo {
    pub inner: Decimal,
    pub test: Py<Test>,
}

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Bar {
    pub foo: Py<Foo>,
}

#[pymethods]
impl Bar {
    #[new]
    fn new() -> PyResult<Bar> {
        Python::with_gil(|py| {
            let foo: Py<Foo> = Py::new(
                py,
                Foo {
                    inner: Decimal::from_f64(24.1).unwrap(),
                    test: Py::new(
                        py,
                        Test {
                            mine: Decimal::from_f64(24.1).unwrap(),
                        },
                    )?,
                },
            )?;
            Ok(Bar { foo })
        })
    }

    fn test(&self, pdf: PyDataFrame) {
        let val = self.foo.get().inner;
        println!("Za val is this {:?}", val);
        // Python::with_gil(|py| {
        //     let val = self.foo.borrow(py);
        //     println!("Za val is this {:?}", val);
        // });
    }
}

#[pyclass(get_all)]
#[derive(Debug)]
pub struct BT {
    pub strategy: Py<Strategy>,
}

#[pymethods]
impl BT {
    #[new]
    fn new(strategy: Py<Strategy>) -> PyResult<BT> {
        Ok(BT { strategy })
    }

    fn test(&self) {
        Python::with_gil(|py| {
            // Access the Python class object of Strategy
            self.strategy
                .call_method_bound(py, "major", (), None)
                .unwrap();
            let val = self.strategy.borrow(py).val;
            println!("Za val is this {:?}", val);
        });
    }
}

// fn backtest(&self) {
//     let df = self.df.0.clone();
//     for (i, col) in df.get_columns().iter().enumerate() {
//         // Get the column name
//         let name = col.name();
//         // println!("Column name: {}", name);

//         Python::with_gil(|py| {
//             let kwargs_dict = PyDict::new_bound(py);
//             // // Access each value in the column
//             // for (index, value) in col.iter().enumerate() {
//             //     // println!("Row {}: {:?}", index, value);
//             //     let val = value.extract::<f64>().unwrap();
//             //     kwargs_dict
//             //         .set_item(index.to_string(), value.extract::<f64>().unwrap())
//             //         .unwrap();
//             // }
//             let result: Decision = self
//                 .strategy
//                 .call_method_bound(
//                     py,
//                     "on_candle_col",
//                     (name.to_string(), self.has_position),
//                     Some(&kwargs_dict),
//                 )
//                 .unwrap()
//                 .extract(py)
//                 .unwrap();
//             // println!("Za val is this {:?}", result);
//         });
//     }
// }
// fn backtest_with_row(&self) {
//     let df = self.df.0.clone();
//     let column_names = df.get_column_names();
//     for i in 0..df.height() {
//         // let row: Row = self.df.0.get_row(i).unwrap();
//         // let mut kwargs = Vec::new();
//         // for (index, col_name) in column_names.iter().enumerate() {
//         //     // println!(
//         //     //     "Column index: {}, Column name: {:?}, Column value: {:?}",
//         //     //     index,
//         //     //     col_name,
//         //     //     row.0.get(index).unwrap()
//         //     // );

//         //     kwargs.push((
//         //         col_name,
//         //         row.0.get(index).unwrap().extract::<f64>().unwrap(),
//         //     ));
//         // }
//         // println!("{:?}", val);
//         // println!("LETS GO");
//         // let _row = PyList(row.0.get(0).unwrap());
//         Python::with_gil(|py| {
//             // Access the Python class object of Strategy
//             // let kwargs = [("open", val.into_py(py)), ("close", val)].into_py_dict_bound(py);
//             // let kwargs = [
//             //     ("open", val.extract::<f64>().unwrap()),
//             //     ("high", val.extract::<f64>().unwrap()),
//             //     ("low", val.extract::<f64>().unwrap()),
//             //     ("close", val.extract::<f64>().unwrap()),
//             // ]
//             // .into_py_dict_bound(py);
//             //
//             // let val = row.0.get("open");
//             // println!("Za val is this {:?}", val);
//             // let kwargs_dict = PyDict::new_bound(py);
//             // kwargs_dict
//             //     .set_item("cross_above", row.0.get(i))
//             //     .unwrap();
//             // for (key, value) in kwargs {
//             //     // for (index, col_name) in self.df.0.get_column_names().iter().enumerate() {
//             //     kwargs_dict.set_item("ASD", value).unwrap();
//             //     // kwargs_dict.set_item(col_name.as_ref(), 3).unwrap();
//             // }
//             let result: Decision = self
//                 .strategy
//                 // .call_method_bound(py, "on_candle", (i, self.has_position), Some(&kwargs_dict))
//                 .call_method_bound(py, "on_candle", (i,), None)
//                 .unwrap()
//                 .extract(py)
//                 .unwrap();
//             // println!("Za val is this {:?}", result);
//         });
//         // if i > 2 {
//         //     break;
//         // }
//     }
// }
//
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

// ==== On passing dict via py actions
// for key in py_actions.keys() {
//     match key.as_str() {
//         "new_position" => {
//             if let Some(new_position) = py_actions.get(key).unwrap().downcast_ref::<Position>()
//             {
//                 // // println!("The new position is: {:?}", new_position);
//                 // // Add position to the state safely
//                 // set_state_dict_item(
//                 //     _pystate,
//                 //     "active_positions",
//                 //     new_position.id.clone(),
//                 //     new_position.clone().into_py(py), // Convert to a Python object
//                 // );
//                 // let mut pystate = _pystate.borrow_mut(py);
//                 // pystate.active_position = Some(new_position.clone());
//             } else {
//                 // Handle the case where the downcast failed
//                 println!("Error: The value associated with the key is not a `Position`.");
//             }
//         }
//         "close_positions" => {
//             // if let Some(positions_to_close) =
//             //     py_actions.get(key).unwrap().downcast_ref::<Vec<String>>()
//             // {
//             //     // println!("The positions to close are: {:?}", positions_to_close);
//             //     for pos_id in positions_to_close {
//             //         remove_state_dict_item(_pystate, "active_positions", pos_id);
//             //     }
//             // }
//         }
//         _ => println!("Unknown key {:?}", key),
//     }
// }
// append_decimal_to_list(_pystate, "equity", state.equity.last().unwrap().clone());
