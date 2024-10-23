// use super::ohlc::OHLC;
// use super::order::Order;
// use super::position::Position;
use crate::backtest::backtest_params::BacktestParams;
use crate::sdk::order::Order;
use crate::sdk::position::Position;
use chrono::{DateTime, Utc};
use rand::Rng; // Import the Rng trait
use rust_decimal_macros::dec;

pub fn create_position(order: &Order, date: DateTime<Utc>, params: &BacktestParams) -> Position {
    let entry_price = order.price.expect("Order price is None!");

    Position {
        id: rand::thread_rng().gen_range(0..101).to_string(),
        index: order.index,
        exit_index: 0,
        entry_timestamp: date,
        exit_timestamp: None,
        entry_price,
        exit_price: None,
        size: order.size,
        sl: order.sl,
        tp: order.tp,
        side: order.side,
        close_reason: None,
        pnl: dec!(0.0),
        max_dd: dec!(0.0),
        commission: entry_price * params.commission_pct * order.size,
        commission_pct: params.commission_pct,
    }
}
