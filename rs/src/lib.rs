mod backtest;
mod helpers;
mod indicators;
mod sdk;
mod strategy;

use backtest::backtester::Backtest;
use backtest::params::BacktestParams;
use backtest::shared_state::{PySharedState, SharedState};
use pyo3::prelude::*;

use strategy::actions::Action;
use strategy::base::Strategy;

use indicators::cross_above::cross_above;
use indicators::cross_below::cross_below;
use sdk::backtest::BacktestOld;
use sdk::bbo::BBO;
use sdk::contract::Contract;
use sdk::enums::{OrderType, Side};
use sdk::ohlc::OHLCs;
use sdk::order::{LimitOrders, Order};
use sdk::position::Position;
use sdk::signal::Signal;
use sdk::signals::Signals;

#[pymodule]
fn rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(round_value, m)?)?;
    m.add_class::<Signal>()?;

    // m.add_class::<Instrument>()?;
    m.add_class::<Strategy>()?;
    m.add_class::<Action>()?;

    m.add_class::<Backtest>()?;
    m.add_class::<SharedState>()?;
    m.add_class::<PySharedState>()?;
    m.add_class::<BacktestOld>()?;
    // m.add_class::<BT>()?;
    // m.add_class::<Bar>()?;
    // m.add_class::<Foo>()?;

    m.add_class::<Contract>()?;
    m.add_class::<BBO>()?;
    m.add_class::<OHLCs>()?;
    m.add_class::<Backtest>()?;
    m.add_class::<BacktestParams>()?;
    m.add_class::<Signals>()?;
    m.add_class::<Signal>()?;
    m.add_class::<Position>()?;
    m.add_class::<LimitOrders>()?;
    m.add_class::<Side>()?;
    m.add_class::<OrderType>()?;
    m.add_class::<Order>()?;

    m.add_function(wrap_pyfunction!(cross_above, m)?)?;
    m.add_function(wrap_pyfunction!(cross_below, m)?)?;
    Ok(())
}
