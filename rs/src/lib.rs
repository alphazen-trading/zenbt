mod backtest;
mod indicators;
mod sdk;
mod strategy;

use backtest::backtester::Backtest;
use backtest::params::BacktestParams;
use backtest::shared_state::{PySharedState, SharedState};
use backtest::stats::Stats;
use pyo3::prelude::*;

use strategy::actions::Action;
use strategy::base::Strategy;

use indicators::cross_above::cross_above;
use indicators::cross_below::cross_below;
use indicators::indicator_123::indicator_123;
use sdk::enums::{CloseReason, OrderType, Side};
use sdk::order::{LimitOrders, Order};
use sdk::position::Position;

fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "indicators")?;
    child_module.add_function(wrap_pyfunction!(cross_above, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(cross_below, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(indicator_123, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

#[pymodule]
fn zbt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;

    m.add_class::<Strategy>()?;
    m.add_class::<Action>()?;

    m.add_class::<Backtest>()?;
    m.add_class::<SharedState>()?;
    m.add_class::<PySharedState>()?;

    m.add_class::<Backtest>()?;
    m.add_class::<BacktestParams>()?;
    m.add_class::<Position>()?;
    m.add_class::<LimitOrders>()?;
    m.add_class::<Side>()?;
    m.add_class::<CloseReason>()?;
    m.add_class::<OrderType>()?;
    m.add_class::<Order>()?;
    m.add_class::<Stats>()?;

    // m.add_function(wrap_pyfunction!(round_value, m)?)?;
    Ok(())
}
