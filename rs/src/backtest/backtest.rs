use super::backtest_methods::create_position;
use super::backtest_params::BacktestParams;
use super::shared_state::SharedState;
use crate::sdk::enums::OrderType;
use crate::sdk::position::Position;
use crate::sdk::position::Positions;
use crate::strategy::actions::Action;
use crate::strategy::strategy::Strategy;
use chrono::{DateTime, NaiveDateTime, Utc};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

// use super::ohlc::OHLC;
use pyo3_polars::PyDataFrame;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::borrow::BorrowMut;

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Backtest {
    pub df: PyDataFrame,
    pub data: Py<PyDict>,
    pub backtest_params: BacktestParams,
    pub strategy: Py<Strategy>,
    pub state: Py<SharedState>,
}

#[pymethods]
impl Backtest {
    #[new]
    fn new(
        df: PyDataFrame,
        backtest_params: BacktestParams,
        strategy: Py<Strategy>,
    ) -> PyResult<Backtest> {
        Python::with_gil(|py| {
            let data = strategy.borrow(py).data.clone_ref(py);
            println!("Za val is this {:?}", df);
            Ok(Backtest {
                df,
                data,
                backtest_params,
                strategy,
                state: Py::new(
                    py,
                    SharedState {
                        equity: PyList::new_bound(py, Vec::<f64>::new()).into(),
                        active_positions: PyList::new_bound(py, Vec::<Position>::new()).into(),
                        closed_positions: PyList::new_bound(py, Vec::<Position>::new()).into(),
                    },
                )?,
            })
        })
    }

    fn _append_to_list(&self, list_name: &str, value: PyObject) {
        Python::with_gil(|py| {
            let mut list = self.state.getattr(py, list_name).unwrap();
            list.borrow_mut()
                .call_method_bound(py, "append", (value,), None)
                .unwrap();
        });
    }

    fn backtest(&self) {
        let df = self.df.0.clone();
        for i in 0..df.height() {
            // self._append_to_list("equity", 1);
            let action = Python::with_gil(|py| {
                let state = self.state.borrow(py);
                let result: Py<Action> = self
                    .strategy
                    // .call_method_bound(py, "on_candle", (i, self.has_position), Some(&kwargs_dict))
                    .call_method_bound(py, intern!(py, "_on_candle"), (i, state), None)
                    .unwrap()
                    .extract(py)
                    .unwrap();
                result.extract::<Action>(py).unwrap()
            });

            let active_positions: Vec<Position> =
                Python::with_gil(|py| self.state.borrow(py).active_positions.extract(py).unwrap());

            for mut order in action.desired_orders {
                if order.order_type == OrderType::Market {
                    let price: f64 = df["open"].get(i + 1).unwrap().try_extract::<f64>().unwrap();

                    // Extract the timestamp (assuming it's in seconds or milliseconds) as an i64
                    let timestamp_ms: i64 = df["time"]
                        .get(i + 1)
                        .unwrap()
                        .try_extract::<i64>() // Extract the date as an Int64 (timestamp)
                        .unwrap();

                    // Convert milliseconds to seconds and nanoseconds
                    let seconds = timestamp_ms / 1_000;
                    let nanoseconds = (timestamp_ms % 1_000) * 1_000_000;

                    // Create a NaiveDateTime from the seconds and nanoseconds
                    let naive_dt = NaiveDateTime::from_timestamp(seconds, nanoseconds as u32); // timestamp in seconds and nanoseconds

                    // Convert the NaiveDateTime to DateTime<Utc>
                    let date: DateTime<Utc> = DateTime::<Utc>::from_utc(naive_dt, Utc);

                    order.price = Decimal::from_f64(price);
                    // println!("Za val is this {} {:?}", i, order.price);
                    // println!("Za val is this {} {:?}", i, df["open"].get(i + 1));

                    let new_position = create_position(&order, date, &self.backtest_params);
                    Python::with_gil(|py| {
                        self._append_to_list("active_positions", new_position.into_py(py));
                    })
                }
                // println!("{:?}", order);
            }

            for position in active_positions {
                println!("The active position: {:?}", position);
            }
        }
    }
}
