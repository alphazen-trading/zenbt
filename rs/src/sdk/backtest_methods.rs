use super::backtest_params::BacktestParams;
use super::ohlc::OHLC;
use super::order::Order;
use super::position::Position;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub fn was_order_hit(ohlc: &OHLC, order: &Order) -> bool {
    if order.side == dec!(1.0) {
        // if ohlc.low <= order.price {
        //     println!("ORDER WAS HIT");
        //     println!("{:?}", ohlc.low);
        //     println!("{:?} {:?}", order.price, order.sl);
        // }
        return ohlc.low <= order.price;
    } else {
        // if ohlc.high >= order.price {
        //     if ohlc.high >= order.sl {
        //         println!("\nORDER WAS HIT BUT Problem with sl");
        //         println!("{:?}", ohlc);
        //         println!("{:?}", order);
        //     }
        // }
        return ohlc.high >= order.price;
    }
}

pub fn has_account_blown_up(equity: &Vec<Decimal>, floating_equity: &Vec<Decimal>) -> bool {
    return equity.last().unwrap() + floating_equity.last().unwrap() < dec!(0.0);
}

pub fn create_position(order: &Order, ohlc: &OHLC, params: &BacktestParams) -> Position {
    Position {
        index: order.index,
        entry_timestamp: ohlc.date,
        exit_timestamp: None,
        entry_price: order.price,
        exit_price: None,
        size: order.size,
        sl: order.sl,
        tp: order.tp,
        side: order.side,
        close_reason: None,
        pnl: dec!(0.0),
        max_dd: dec!(0.0),
        commission: order.price * params.commission_pct * order.size,
        commission_pct: params.commission_pct,
    }
}
