mod helpers;
mod indicators;
mod sdk;

use crate::helpers::round_value::round_value;
use indicators::cross_above::cross_above;
use indicators::cross_below::cross_below;
use pyo3::prelude::*;
use sdk::backtest::Backtest;
use sdk::backtest_params::BacktestParams;
use sdk::bbo::BBO;
use sdk::contract::Contract;
use sdk::instrument::Instrument;
use sdk::ohlc::OHLC;
use sdk::order::Order;
use sdk::position::Position;
use sdk::signals::Signals;

/// A Python module implemented in Rust.
#[pymodule]
fn rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(round_value, m)?)?;
    m.add_class::<Instrument>()?;
    m.add_class::<Contract>()?;
    m.add_class::<BBO>()?;
    m.add_class::<OHLC>()?;
    m.add_class::<Backtest>()?;
    m.add_class::<BacktestParams>()?;
    m.add_class::<Signals>()?;
    m.add_class::<Position>()?;
    m.add_class::<Order>()?;

    m.add_function(wrap_pyfunction!(cross_above, m)?)?;
    m.add_function(wrap_pyfunction!(cross_below, m)?)?;
    Ok(())
}
