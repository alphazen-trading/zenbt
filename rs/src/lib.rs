mod helpers;
mod sdk;
use crate::helpers::round_value::round_value;
use pyo3::prelude::*;
use sdk::bbo::BBO;
use sdk::contract::Contract;
use sdk::instrument::Instrument;
use sdk::pnl::PNL;
use sdk::ohlc::OHLC;
use sdk::backtest::Backtest;
use sdk::signals::Signals;

/// A Python module implemented in Rust.
#[pymodule]
fn rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(round_value, m)?)?;
    m.add_class::<Instrument>()?;
    m.add_class::<Contract>()?;
    m.add_class::<BBO>()?;
    m.add_class::<PNL>()?;
    m.add_class::<OHLC>()?;
    m.add_class::<Backtest>()?;
    m.add_class::<Signals>()?;
    Ok(())
}
