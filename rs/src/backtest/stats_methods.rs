// use std::time::Instant;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::{backtester::Backtest, helpers::get_date_at_index, stats::Stats};

pub fn calculate_max_drawdown(values: &[Decimal]) -> Option<Decimal> {
    if values.is_empty() {
        return None;
    }

    let mut max_drawdown = dec!(0.0);
    let mut peak = values[0];

    for &value in values {
        if value > peak {
            peak = value;
        } else {
            let drawdown = (peak - value).abs();
            max_drawdown = max_drawdown.max(drawdown);
        }
    }

    Some(max_drawdown)
}

pub fn create_stats(backtest: &Backtest) -> Stats {
    let mut wins = dec!(0);
    let mut losses = dec!(0);
    let df = backtest.df.0.clone();
    for position in backtest.state.closed_positions.values() {
        if position.pnl > dec!(0.0) {
            wins += dec!(1);
        } else {
            losses += dec!(1);
        }
    }
    let mut commissions = backtest.commissions;
    for position in backtest.state.active_positions.values() {
        commissions += position.commission;
    }

    let mut win_rate = dec!(0);
    if wins + losses > dec!(0) {
        win_rate = (wins / (wins + losses) * dec!(100.0)).round_dp(2);
    }

    let max_drawdown = calculate_max_drawdown(&backtest.state.equity).unwrap_or(dec!(0.0));
    let pnl = backtest.state.equity.last().unwrap() - backtest.params.initial_capital
        + backtest.state.floating_equity.last().unwrap();

    let start_date = get_date_at_index(&df, 0);
    let end_date = get_date_at_index(&df, df.height() - 1);

    let stats = Stats {
        initial_capital: backtest.params.initial_capital,
        pnl,
        pnl_pct: pnl * dec!(100) / backtest.params.initial_capital,
        unrealized_pnl: *backtest.state.floating_equity.last().unwrap(),
        total_positions: backtest.state.active_positions.len()
            + backtest.state.closed_positions.len(),
        closed_positions: backtest.state.closed_positions.len(),
        active_positions: backtest.state.active_positions.len(),
        commissions,
        wins,
        losses,
        win_rate: win_rate.to_string() + "%",
        trading_days: end_date.signed_duration_since(start_date).num_days(),
        // trading_days: backtest
        //     .ohlc
        //     .last()
        //     .unwrap()
        //     .date
        //     .signed_duration_since(backtest.ohlc.first().unwrap().date)
        //     .num_days(),
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        max_drawdown,
        max_drawdown_pct: max_drawdown * dec!(100) / backtest.params.initial_capital,
    };

    stats
}
