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
