mod helpers;
mod indicators;
mod sdk;

// use crate::helpers::create_limit_orders::create_limit_orders;
// use crate::helpers::create_signals::create_signals;
// use crate::helpers::round_value::round_value;
// use ndarray::Ix1;
// use indicators::cross_above::cross_above;
// use indicators::cross_below::cross_below;
use pyo3::prelude::*;
// use sdk::backtest::Backtest;
// use sdk::backtest_params::BacktestParams;
// use sdk::bbo::BBO;
// use sdk::contract::Contract;
// use sdk::enums::{OrderType, Side};
// use sdk::instrument::Instrument;
use sdk::ohlc::OHLCs;
// use sdk::order::LimitOrders;
// use sdk::position::Position;
use sdk::signal::Signal;
// use sdk::signals::Signals;
// use sdk::strategy::{Bar, Foo, Strategy, BT};

#[pymodule]
fn rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(round_value, m)?)?;
    // m.add_function(wrap_pyfunction!(create_limit_orders, m)?)?;
    // m.add_function(wrap_pyfunction!(create_signals, m)?)?;
    m.add_class::<Signal>()?;

    // m.add_class::<Instrument>()?;
    // m.add_class::<Strategy>()?;
    // m.add_class::<BT>()?;
    // m.add_class::<Bar>()?;
    // m.add_class::<Foo>()?;

    // m.add_class::<Contract>()?;
    // m.add_class::<BBO>()?;
    m.add_class::<OHLCs>()?;
    // m.add_class::<Backtest>()?;
    // m.add_class::<BacktestParams>()?;
    // m.add_class::<Signals>()?;
    // m.add_class::<Signal>()?;
    // m.add_class::<Position>()?;
    // m.add_class::<LimitOrders>()?;
    // m.add_class::<Side>()?;
    // m.add_class::<OrderType>()?;

    // m.add_function(wrap_pyfunction!(cross_above, m)?)?;
    // m.add_function(wrap_pyfunction!(cross_below, m)?)?;
    Ok(())
}
