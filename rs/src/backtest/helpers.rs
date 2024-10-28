use chrono::{DateTime, NaiveDateTime, Utc};
use polars::frame::DataFrame;
use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::borrow::BorrowMut;

use super::backtest::Backtest;
use super::shared_state::PySharedState;

pub fn get_date_at_index(df: &DataFrame, index: usize) -> DateTime<Utc> {
    let timestamp_ms: i64 = df["time"]
        .get(index + 1)
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
    date
}

pub fn append_decimal_to_list(pystate: &Py<PySharedState>, list_name: &str, value: Decimal) {
    Python::with_gil(|py| {
        let mut list = pystate.getattr(py, list_name).unwrap();
        list.borrow_mut()
            .call_method_bound(py, "append", (value,), None)
            .unwrap();
    });
}

pub fn append_to_list(pystate: &Py<PySharedState>, list_name: &str, value: PyObject) {
    Python::with_gil(|py| {
        let mut list = pystate.getattr(py, list_name).unwrap();
        list.borrow_mut()
            .call_method_bound(py, "append", (value,), None)
            .unwrap();
    });
}

// pub fn set_state_dict_item(backtest: &Backtest, dict_name: &str, key: String, value: PyObject) {
//     Python::with_gil(|py| {
//         let binding = backtest.pystate.getattr(py, dict_name).unwrap();
//         let mut _binding = binding.bind(py);
//         let dict = _binding.borrow_mut();
//         dict.set_item(key, value).unwrap();
//     });
// }

pub fn set_state_dict_item(
    pystate: &Py<PySharedState>,
    dict_name: &str,
    key: String,
    value: PyObject,
) {
    Python::with_gil(|py| {
        let binding = pystate.getattr(py, dict_name).unwrap();
        let mut _binding = binding.bind(py);
        let dict = _binding.borrow_mut();
        dict.set_item(key, value).unwrap();
    });
}

pub fn remove_state_dict_item(pystate: &Py<PySharedState>, dict_name: &str, key: &String) {
    Python::with_gil(|py| {
        let binding = pystate.getattr(py, dict_name).unwrap();
        let mut _binding = binding.bind(py);
        let dict = _binding.borrow_mut();
        dict.del_item(key).unwrap();
    });
}

pub fn get_value_at(df: &DataFrame, index: usize, column: &str) -> Option<Decimal> {
    let price: f64 = df[column].get(index).unwrap().try_extract::<f64>().unwrap();
    Some(Decimal::from_f64(price).unwrap())
}
