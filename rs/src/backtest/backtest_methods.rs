use crate::backtest::helpers::{get_date_at_index, get_value_at};
use crate::sdk::enums::CloseReason;
use crate::sdk::position::Position;
use polars::frame::DataFrame;
use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::borrow::BorrowMut;
use std::collections::HashMap;

use super::shared_state::SharedState;

pub fn check_positions_to_close(
    i: usize,
    df: &DataFrame,
    // active_positions: *mut Py<PyAny>,
    // closed_positions: *mut Py<PyAny>,
    state: &Py<SharedState>,
    desired_positions: HashMap<String, Position>,
) {
    // let mut positions_to_remove = Vec::new();

    Python::with_gil(|py| {
        let binding = state.getattr(py, "active_positions").unwrap();
        let mut _binding = binding.bind(py);
        let dict = _binding.borrow_mut();
        // dict.set_item("asd", 3.2);

        // for key in dict.keys() {
        //     if !desired_positions.contains_key(&key) {}
        // }

        // for pos in active_positions {
        //     pos.0.
        //     if !desired_positions.contains_key(&pos.id) {
        //         pos.close_position(
        //             i,
        //             get_date_at_index(&df, i),
        //             get_value_at(&df, i + 1, "open").unwrap(),
        //             CloseReason::Manual,
        //         );
        //         closed_positions.insert(pos.id.clone(), pos.clone());
        //         positions_to_remove.push(pos.id.clone());
        //     }
        // }
    })

    // // Now remove the positions after iteration
    // for id in positions_to_remove {
    //     active_positions.remove(&id);
    // }
}

// pub fn update_backtest_equity(
//     backtest: &mut BacktestOld,
//     floating_equity: Decimal,
//     realized_equity: Decimal,
// ) {
//     backtest.floating_equity.push(floating_equity);
//     backtest.equity.push(
//         backtest
//             .equity
//             .last()
//             .unwrap_or(&backtest.params.initial_capital)
//             + realized_equity,
//     );
// }
