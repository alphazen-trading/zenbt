use crate::{
    sdk::{
        enums::{OrderType, Side},
        order::Order,
        position::Position,
    },
    strategy::actions::Action,
};
use polars::frame::DataFrame;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::{
    backtester::Backtest,
    helpers::{get_date_at_index, get_value_at},
};

pub fn check_positions_to_close(
    i: usize,
    df: &DataFrame,
    backtest: &mut Backtest,
    action: &Action,
    // state: &mut RustState, // py_actions: &mut HashMap<String, Box<dyn Any>>,
) {
    let state = &mut backtest.state;
    let mut positions_to_close: Vec<String> = Vec::new();
    let mut floating_equity = dec!(0);
    let mut realized_equity = dec!(0);

    for position in &mut state.active_positions.values_mut() {
        let should = position.should_close(i, df, action);
        if should {
            state
                .closed_positions
                .insert(position.id.clone(), position.clone());
            backtest.commissions += position.commission;
            realized_equity += position.pnl;
            // println!("{i} -- closing position pnl {}", position.pnl);
            // println!("closing position pnl {:?}", position.close_reason.unwrap());
            positions_to_close.push(position.id.clone());
        } else {
            position.update_pnl(i, df);
            floating_equity += position.pnl;
        }
    }

    if !positions_to_close.is_empty() {
        for pos_id in &positions_to_close {
            state.active_positions.remove(pos_id);
        }
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
            .unwrap_or(&backtest.params.initial_capital)
            + realized_equity,
    );
}

pub fn was_order_hit(order: &Order, i: usize, df: &DataFrame) -> bool {
    let low = get_value_at(df, i, "low");
    let high = get_value_at(df, i, "high");
    let price = order.price.unwrap();
    if price > low && price < high {
        return true;
    }
    false
    // if order.order_type == OrderType::Limit {
    //     if order.side == Side::Short {
    //         let low = get_value_at(df, i, "low");
    //         low <= order.price.unwrap()
    //     } else {
    //         let high = get_value_at(df, i, "high");
    //         high <= order.price.unwrap()
    //     }
    // } else if order.order_type == OrderType::Stop {
    //     if order.side == Side::Short {
    //         let low = get_value_at(df, i, "low");
    //         low <= order.price.unwrap()
    //     } else {
    //         let high = get_value_at(df, i, "high");
    //         high <= order.price.unwrap()
    //     }
    // } else {
    //     false
    // }
}

pub fn was_pending_order_triggered(
    order: &Order,
    i: usize,
    df: &DataFrame,
    backtest: &mut Backtest,
) -> bool {
    if was_order_hit(order, i, df) {
        // println!("{i} Position filled");
        let mut new_position =
            Position::create_position(order, get_date_at_index(df, i), &backtest.params);

        if new_position.was_sl_hit(i, df) {
            // If SL was hit in the same candle, update equity and move to closed positions

            if backtest.params.verbose {
                println!("{i} Wanting to fill a pending order, but SL was hit.");
            }
            if let Some(last_equity) = backtest.state.equity.last_mut() {
                *last_equity += new_position.pnl;
            }
            backtest
                .state
                .closed_positions
                .insert(new_position.id.clone(), new_position.clone());
        } else {
            // If SL wasn't hit, move the position to active positions
            if backtest.params.verbose {
                println!("{i} Adding new order: {:#?}", order);
            }
            backtest
                .state
                .active_positions
                .insert(new_position.id.clone(), new_position.clone());
        }
        return true;
    }
    false
}
