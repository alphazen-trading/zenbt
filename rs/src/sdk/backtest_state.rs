use std::time::Instant;

use super::backtest::Backtest_old;
use super::stats_methods::create_stats;
use pyo3::prelude::*;

use pyo3::types::PyDict;

pub fn get_state(backtest: &Backtest_old, py: Python) -> PyResult<PyObject> {
    let start = Instant::now();
    let dict = PyDict::new_bound(py);

    dict.set_item("commission_pct", backtest.params.commission_pct)?;
    dict.set_item("commissions", backtest.commissions)?;
    dict.set_item("initial_capital", backtest.params.initial_capital)?;
    dict.set_item(
        "active_positions",
        backtest.positions.active_positions.clone(),
    )?;
    dict.set_item(
        "closed_positions",
        backtest.positions.closed_positions.clone(),
    )?;
    dict.set_item("equity", backtest.equity.clone())?;
    dict.set_item("floating_equity", backtest.floating_equity.clone())?;
    dict.set_item("stats", create_stats(&backtest))?;

    let duration = start.elapsed();

    println!("RS Time elapsed: {:?}", duration);

    Ok(dict.into())
}
