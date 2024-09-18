use super::ohlc::OHLC;
use super::order::Order;
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
