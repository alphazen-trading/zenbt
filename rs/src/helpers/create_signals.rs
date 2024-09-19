use numpy::{PyArrayDyn, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::Bound;
use rust_decimal::Decimal;

use crate::sdk::{
    enums::Side,
    signal::{Signal, Signals},
};
use rust_decimal::prelude::FromPrimitive;
use std::collections::HashMap;

#[pyfunction]
pub fn create_signals<'py>(
    long_entries: &Bound<'py, PyArrayDyn<bool>>,
    long_exits: &Bound<'py, PyArrayDyn<bool>>,
    short_entries: &Bound<'py, PyArrayDyn<bool>>,
    short_exits: &Bound<'py, PyArrayDyn<bool>>,
) -> Signals {
    let mut signals: Signals = HashMap::new();

    let long_entries = unsafe { long_entries.as_array_mut() };
    let long_exits = unsafe { long_exits.as_array_mut() };
    let short_entries = unsafe { short_entries.as_array_mut() };
    let short_exits = unsafe { short_exits.as_array_mut() };

    fn add_signal(signals: &mut Signals, index: Decimal, new_signal: Signal) {
        match signals.get_mut(&index) {
            Some(vec) => vec.push(new_signal),
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(new_signal);
                signals.insert(index, new_vec);
            }
        }
    }

    for i in 0..long_entries.len() {
        let index = Decimal::from_f64(i as f64).unwrap();
        if long_entries[i] {
            let new_signal = Signal {
                index: Decimal::from_f64(i as f64).unwrap(),
                side: Side::Long,
                signal_type: String::from("open"),
                order_type: String::from("market"),
            };
            add_signal(&mut signals, index, new_signal);
        }
        if long_exits[i] {
            let new_signal = Signal {
                index: Decimal::from_f64(i as f64).unwrap(),
                side: Side::Short,
                signal_type: String::from("close"),
                order_type: String::from("market"),
            };
            add_signal(&mut signals, index, new_signal);
        }
        if short_entries[i] {
            let new_signal = Signal {
                index: Decimal::from_f64(i as f64).unwrap(),
                side: Side::Short,
                signal_type: String::from("open"),
                order_type: String::from("market"),
            };
            add_signal(&mut signals, index, new_signal);
        }
        if short_exits[i] {
            let new_signal = Signal {
                index: Decimal::from_f64(i as f64).unwrap(),
                side: Side::Long,
                signal_type: String::from("close"),
                order_type: String::from("market"),
            };
            add_signal(&mut signals, index, new_signal);
        }
    }
    signals
}
