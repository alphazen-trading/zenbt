use super::backtest_params::BacktestParams;
use numpy::ToPyArray;
use polars::prelude::*;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyType;

// use super::ohlc::OHLC;
use super::enums::Decision;
use super::position::Positions;
use pyo3_polars::PyDataFrame;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass(get_all, set_all, subclass)]
#[derive(Debug)]
pub struct Strategy {
    pub df: PyDataFrame,
    pub data: Py<PyDict>,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub backtest_params: BacktestParams,
    pub positions: Positions,
}

#[pymethods]
impl Strategy {
    #[new]
    fn new(df: PyDataFrame, backtest_params: BacktestParams) -> PyResult<Strategy> {
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
                floating_equity: Vec::new(),
                backtest_params,
                positions: Positions::new(),
            })
        })
    }

    #[classmethod]
    pub fn on_candle(cls: &Bound<'_, PyType>) -> PyResult<i32> {
        Ok(10)
    }
}

#[pyclass(get_all, frozen)]
#[derive(Debug)]
pub struct Result {
    pub test: f64,
}

#[pymethods]
impl Result {
    #[new]
    fn new(test: f64) -> PyResult<Result> {
        Ok(Result { test })
    }
}

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Backtest {
    pub df: PyDataFrame,
    pub equity: Vec<Decimal>,
    pub floating_equity: Vec<Decimal>,
    pub backtest_params: BacktestParams,
    pub positions: Positions,
    pub strategy: Py<Strategy>,
    pub has_position: bool,
}

#[pymethods]
impl Backtest {
    #[new]
    fn new(
        df: PyDataFrame,
        backtest_params: BacktestParams,
        strategy: Py<Strategy>,
    ) -> PyResult<Backtest> {
        Ok(Backtest {
            df,
            equity: Vec::new(),
            floating_equity: Vec::new(),
            backtest_params,
            positions: Positions::new(),
            strategy,
            has_position: false,
        })
    }

    fn backtest_with_row(&self) {
        let df = self.df.0.clone();
        for i in 0..df.height() {
            Python::with_gil(|py| {
                let result: Py<Result> = self
                    .strategy
                    // .call_method_bound(py, "on_candle", (i, self.has_position), Some(&kwargs_dict))
                    .call_method_bound(py, intern!(py, "on_candle"), (i,), None)
                    .unwrap()
                    .extract(py)
                    .unwrap();
                self.strategy.borrow_mut(py).equity.push(dec!(3.4));
                // println!("Za val is this {:?}", result.get());
                // for (key, value) in <pyo3::Py<PyDict> as Into<T>>::into(result) {
                //     println!("Key: {:?}, Value: {:?}", key, value);
                // }
            });
        }
    }
}
