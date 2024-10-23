use super::backtest::BacktestOld;
use super::enums::Side;
use super::ohlc::OHLC;
// use super::order::Order;
use super::position::Position;
use crate::backtest::backtest_params::BacktestParams;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// pub fn was_order_hit(ohlc: &OHLC, order: &Order) -> bool {
//     // if order.side == Side::Long {
//     //     // if ohlc.low <= order.price {
//     //     //     println!("ORDER WAS HIT");
//     //     //     println!("{:?}", ohlc.low);
//     //     //     println!("{:?} {:?}", order.price, order.sl);
//     //     // }
//     //     return ohlc.low <= order.price;
//     // } else {
//     //     // if ohlc.high >= order.price {
//     //     //     if ohlc.high >= order.sl {
//     //     //         println!("\nORDER WAS HIT BUT Problem with sl");
//     //     //         println!("{:?}", ohlc);
//     //     //         println!("{:?}", order);
//     //     //     }
//     //     // }
//     //     return ohlc.high >= order.price;
//     // }
// }

pub fn has_account_blown_up(equity: &Vec<Decimal>, floating_equity: &Vec<Decimal>) -> bool {
    return equity.last().unwrap() + floating_equity.last().unwrap() < dec!(0.0);
}

// pub fn create_position(order: &Order, ohlc: &OHLC, params: &BacktestParams) -> Position {
// Position {
//     index: order.index,
//     exit_index: 0,
//     entry_timestamp: ohlc.date,
//     exit_timestamp: None,
//     entry_price: Some(order.price),
//     exit_price: None,
//     size: order.size,
//     sl: Some(order.sl),
//     tp: Some(order.tp),
//     side: order.side,
//     close_reason: None,
//     pnl: dec!(0.0),
//     max_dd: dec!(0.0),
//     commission: order.price * params.commission_pct * order.size,
//     commission_pct: params.commission_pct,
// }
// }

pub fn create_position_from_signal(
    index: usize,
    ohlc: &OHLC,
    params: &BacktestParams,
    size: Decimal,
    side: Side,
) -> Position {
    Position {
        id: rand::thread_rng().gen_range(0..101).to_string(),
        index,
        exit_index: 0,
        entry_timestamp: ohlc.date,
        exit_timestamp: None,
        entry_price: ohlc.open,
        exit_price: None,
        size,
        sl: None,
        tp: None,
        side,
        close_reason: None,
        pnl: dec!(0.0),
        max_dd: dec!(0.0),
        commission: ohlc.open * params.commission_pct * size,
        commission_pct: params.commission_pct,
    }
}

pub fn update_backtest_equity(
    backtest: &mut BacktestOld,
    floating_equity: Decimal,
    realized_equity: Decimal,
) {
    backtest.floating_equity.push(floating_equity);
    backtest.equity.push(
        backtest
            .equity
            .last()
            .unwrap_or(&backtest.params.initial_capital)
            + realized_equity,
    );
}

pub fn find_signals_to_close(
    i: usize,
    backtest: &mut BacktestOld,
    long_exit: bool,
    short_exit: bool,
) {
    let ohlc = backtest.ohlc[i];
    let mut indexes_to_remove = Vec::new();
    let mut floating_equity = dec!(0);
    let mut realized_equity = dec!(0);

    for (j, position) in &mut backtest.positions.active_positions.iter_mut().enumerate() {
        if long_exit && position.side == Side::Long {
            backtest.positions.closed_positions.push(position.clone());
            backtest.commissions += position.commission;
            realized_equity += position.pnl;
            indexes_to_remove.push(j);
        }
        if short_exit && position.side == Side::Short {
            backtest.positions.closed_positions.push(position.clone());
            backtest.commissions += position.commission;
            realized_equity += position.pnl;
            indexes_to_remove.push(j);
        }
        if !long_exit && !short_exit {
            // position.tp = backtest.trailing_tp[i];
            position.update_pnl(ohlc.close);
            floating_equity += position.pnl;
        }
    }
    for &i in indexes_to_remove.iter().rev() {
        backtest.positions.active_positions.remove(i);
    }
    update_backtest_equity(backtest, floating_equity, realized_equity);
}

pub fn find_signals_to_enter(
    i: usize,
    backtest: &mut BacktestOld,
    long_entry: bool,
    short_entry: bool,
) {
    let ohlc = backtest.ohlc[i];
    let size = dec!(1.0);
    if long_entry {
        let new_position =
            create_position_from_signal(i, &ohlc, &backtest.params, size, Side::Long);
        backtest.positions.active_positions.push(new_position);
    }
    if short_entry {
        let new_position =
            create_position_from_signal(i, &ohlc, &backtest.params, size, Side::Short);
        backtest.positions.active_positions.push(new_position);
    }
}

pub fn find_active_positions_to_close(i: usize, backtest: &mut BacktestOld) {
    let ohlc = &backtest.ohlc[i];
    let mut indexes_to_remove = Vec::new();
    let mut floating_equity = dec!(0);
    let mut realized_equity = dec!(0);

    for (j, position) in &mut backtest.positions.active_positions.iter_mut().enumerate() {
        let should = position.should_close(i, &ohlc);
        if should {
            backtest.positions.closed_positions.push(position.clone());
            backtest.commissions += position.commission;
            realized_equity += position.pnl;
            indexes_to_remove.push(j);
        } else {
            // position.tp = backtest.trailing_tp[i];
            position.update_pnl(ohlc.close);
            floating_equity += position.pnl;
        }
    }

    for &i in indexes_to_remove.iter().rev() {
        backtest.positions.active_positions.remove(i);
    }

    update_backtest_equity(backtest, floating_equity, realized_equity);
}

#[allow(unused_variables)]
pub fn find_triggered_pending_orders(i: usize, backtest: &mut BacktestOld) {
    // let ohlc = &backtest.ohlc[i];
    // let orders = backtest.limit_orders.get(i);
    // if orders.is_some() {
    //     for order in orders {
    //         if was_order_hit(&ohlc, &order) {
    //             let mut new_position = create_position(&order, ohlc, &backtest.params);

    //             if new_position.was_sl_hit(i, &ohlc) {
    //                 // If SL was hit in the same candle, update equity and move to closed positions
    //                 if let Some(last_equity) = backtest.equity.last_mut() {
    //                     *last_equity += new_position.pnl;
    //                 }
    //                 backtest.positions.closed_positions.push(new_position);
    //             } else {
    //                 // If SL wasn't hit, move the position to active positions
    //                 backtest.positions.active_positions.push(new_position);
    //             }
    //         }
    //     }
    // }
}

// pub fn find_signals_to_manage(i: usize, backtest: &mut Backtest) {
// let ohlc = &backtest.ohlc[i];
// let index = Decimal::from(i);
// let signals = backtest.signals.get(&index);

// let mut indexes_to_remove = Vec::new();
// let mut floating_equity = dec!(0);
// let mut realized_equity = dec!(0);

// if signals.is_some() {
//     for signal in signals.unwrap() {
//         let size = dec!(1);
//         if signal.signal_type == "open" {
//             let new_position = Position {
//                 index,
//                 exit_index: 0,
//                 entry_timestamp: ohlc.date,
//                 exit_timestamp: None,
//                 entry_price: ohlc.open,
//                 exit_price: None,
//                 size,
//                 sl: None,
//                 tp: None,
//                 side: signal.side,
//                 close_reason: None,
//                 pnl: dec!(0.0),
//                 max_dd: dec!(0.0),
//                 commission: ohlc.open * backtest.params.commission_pct * size,
//                 commission_pct: backtest.params.commission_pct,
//             };
//             backtest.positions.active_positions.push(new_position);
//         } else {
//             for (j, position) in &mut backtest.positions.active_positions.iter_mut().enumerate()
//             {
//                 if position.side != signal.side {
//                     position.close_position(
//                         i,
//                         ohlc,
//                         ohlc.close,
//                         CloseReason::Signal,
//                         dec!(0.0),
//                     );
//                     position.exit_timestamp = Some(ohlc.date);
//                     backtest.positions.closed_positions.push(position.clone());
//                     backtest.commissions += position.commission;
//                     realized_equity += position.pnl;
//                     indexes_to_remove.push(j);
//                 }
//             }
//         }
//     }
// }
// for &i in indexes_to_remove.iter().rev() {
//     backtest.positions.active_positions.remove(i);
// }
// for position in &mut backtest.positions.active_positions.iter_mut() {
//     position.update_pnl(ohlc.close);
//     floating_equity += position.pnl;
// }

// update_backtest_equity(backtest, floating_equity, realized_equity);
// }
