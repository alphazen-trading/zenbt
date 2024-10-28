use std::any::Any;
use std::collections::HashMap;

use crate::strategy::actions::Action;
use crate::strategy::strategy::Strategy;
use polars::frame::DataFrame;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::backtest::Backtest;
use super::shared_state::{PySharedState, SharedState};

pub fn test_method(strategy: &mut Strategy) {
    println!("IN here");
    strategy.equity.append(&mut vec![Decimal::from(0)]);
}
pub fn check_positions_to_close(
    i: usize,
    df: &DataFrame,
    backtest: &mut Backtest,
    action: &Action,
    py_actions: &mut HashMap<String, Box<dyn Any>>,
) {
    let state = &mut backtest.state;
    let mut positions_to_close: Vec<String> = Vec::new();
    let mut floating_equity = dec!(0);
    let mut realized_equity = dec!(0);

    for position in &mut state.active_positions.values_mut() {
        let should = position.should_close(i, df);
        if should {
            state
                .closed_positions
                .insert(position.id.clone(), position.clone());
            backtest.commissions += position.commission;
            realized_equity += position.pnl;
            positions_to_close.push(position.id.clone());
        } else {
            position.update_pnl(i, df);
            floating_equity += position.pnl;
        }
    }

    println!("now need to look into the action's desired positions to close and close them");

    if !positions_to_close.is_empty() {
        for pos_id in &positions_to_close {
            state.active_positions.remove(pos_id);
        }
        py_actions.insert("close_positions".to_string(), Box::new(positions_to_close));
    }

    update_backtest_equity(backtest, floating_equity, realized_equity);
}

pub fn update_backtest_equity(
    backtest: &mut Backtest,
    floating_equity: Decimal,
    realized_equity: Decimal,
) {
    backtest.state.floating_equity.push(floating_equity);
    backtest.state.equity.push(
        backtest
            .state
            .equity
            .last()
            .unwrap_or(&backtest.backtest_params.initial_capital)
            + realized_equity,
    );
}
