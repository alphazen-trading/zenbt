use super::shared_state::PySharedState;
use chrono::{DateTime, Utc};
use polars::frame::DataFrame;
use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::borrow::BorrowMut;

pub fn get_date_at_index(df: &DataFrame, index: usize) -> DateTime<Utc> {
    let timestamp_ms: i64 = df["time"]
        .get(index)
        .unwrap()
        .try_extract::<i64>() // Extract the date as an Int64 (timestamp)
        .unwrap();

    // Convert milliseconds to seconds and nanoseconds
    let seconds = timestamp_ms / 1_000;
    let nanoseconds = u32::try_from((timestamp_ms % 1_000) * 1_000_000).expect("Time is invalid");

    // Create a NaiveDateTime from the seconds and nanoseconds
    // let naive_dt = DateTime::from_timestamp(seconds, nanoseconds).unwrap();
    let naive_dt = DateTime::from_timestamp(seconds, nanoseconds).unwrap();

    // Convert the NaiveDateTime to DateTime<Utc>
    // TimeZone.from_utc_datetime(&naive_dt);
    // let date: DateTime<Utc> = DateTime::with_timezone(naive_dt, Utc);
    let date: DateTime<Utc> = naive_dt.with_timezone(&Utc);
    date
}

pub fn get_value_at(df: &DataFrame, index: usize, column: &str) -> Decimal {
    let price: f64 = df[column].get(index).unwrap().try_extract::<f64>().unwrap();
    Decimal::from_f64(price).unwrap()
}

#[allow(dead_code)]
fn append_decimal_to_list(pystate: &Py<PySharedState>, list_name: &str, value: Decimal) {
    Python::with_gil(|py| {
        let mut list = pystate.getattr(py, list_name).unwrap();
        list.borrow_mut()
            .call_method_bound(py, "append", (value,), None)
            .unwrap();
    });
}

#[allow(dead_code)]
fn append_to_list(pystate: &Py<PySharedState>, list_name: &str, value: PyObject) {
    Python::with_gil(|py| {
        let mut list = pystate.getattr(py, list_name).unwrap();
        list.borrow_mut()
            .call_method_bound(py, "append", (value,), None)
            .unwrap();
    });
}

#[allow(dead_code)]
fn set_state_dict_item(pystate: &Py<PySharedState>, dict_name: &str, key: String, value: PyObject) {
    Python::with_gil(|py| {
        let binding = pystate.getattr(py, dict_name).unwrap();
        let mut new_binding = binding.bind(py);
        let dict = new_binding.borrow_mut();
        dict.set_item(key, value).unwrap();
    });
}

#[allow(dead_code)]
fn remove_state_dict_item(pystate: &Py<PySharedState>, dict_name: &str, key: &String) {
    Python::with_gil(|py| {
        let binding = pystate.getattr(py, dict_name).unwrap();
        let mut new_binding = binding.bind(py);
        let dict = new_binding.borrow_mut();
        // println!("\nThe dict: {}", dict);
        // println!("Removing key: {}", key);
        dict.del_item(key).unwrap();
    });
}
