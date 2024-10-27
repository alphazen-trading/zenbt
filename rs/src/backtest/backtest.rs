use super::backtest_methods::check_positions_to_close;
use super::backtest_params::BacktestParams;
use super::helpers::{append_to_list, get_date_at_index, get_value_at, set_state_dict_item};
use super::shared_state::SharedState;
use crate::sdk::enums::OrderType;
use crate::sdk::position::{create_position, Position};
use crate::strategy::actions::Action;
use crate::strategy::strategy::Strategy;
use chrono::{DateTime, NaiveDateTime, Utc};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::any::type_name;
use std::borrow::Borrow;
use std::collections::HashMap;

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
    pub test: SharedState,
    pub active_positions: HashMap<String, Position>,
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
                        active_positions: PyDict::new_bound(py).into(),
                        closed_positions: PyDict::new_bound(py).into(),
                    },
                )?,
                active_positions: HashMap::new(),
            })
        })
    }

    fn backtest(&self) {
        let df = self.df.0.clone();
        // let mut active_positions = Python::with_gil(|py| {
        //     let first = self.state.borrow_mut(py).active_positions;
        //     let mine = Py::clone_ref(&first, py);
        //     mine.bind(py).
        // });
        // let closed_positions = Python::with_gil(|py| -> PyResult<_> {
        //     let item = self.state.getattr(py, "closed_positions").unwrap().extract(py).unwrap();
        //     Ok(item)
        // });

        // let mut active_positions: HashMap<String, Position> =
        //     Python::with_gil(|py| self.state.borrow(py).active_positions.extract(py).unwrap());

        // let mut closed_positions: HashMap<String, Position> =
        //     Python::with_gil(|py| self.state.borrow(py).closed_positions.extract(py).unwrap());
        let mut test: HashMap<String, f64> = HashMap::new();
        test.insert("asdasd".to_string(), 3.3);

        for i in 0..df.height() {
            // self._append_to_list("equity", 1);
            let mut action = Python::with_gil(|py| {
                let result: Py<Action> = self
                    .strategy
                    .call_method_bound(
                        py,
                        intern!(py, "_on_candle"),
                        (i, self.state.borrow(py), test.clone()),
                        None,
                    )
                    .unwrap()
                    .extract(py)
                    .unwrap();
                result.extract::<Action>(py).unwrap()
            });

            check_positions_to_close(
                i,
                &df,
                &self.state,
                // active_positions,
                // closed_positions,
                action.desired_positions,
            );

            // update_backtest_equity();

            for order in action.desired_orders.values_mut() {
                if order.order_type == OrderType::Market {
                    order.price = get_value_at(&df, i + 1, "open");
                    let new_position =
                        create_position(&order, get_date_at_index(&df, i), &self.backtest_params);
                    // self.active_positions
                    //     .insert(new_position.id.clone(), new_position);

                    // Add position to the state
                    Python::with_gil(|py| {
                        set_state_dict_item(
                            self,
                            "active_positions",
                            new_position.id.clone(),
                            new_position.into_py(py),
                        );
                    })
                }
            }

            // for position in active_positions {
            //     // println!("The active position: {:?}", position);
            // }
        }
    }
}
