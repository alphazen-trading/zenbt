use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use super::backtest::Backtest;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    #[serde(with = "rust_decimal::serde::float")]
    pub initial_capital: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub pnl: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub pnl_pct: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub unrealized_pnl: Decimal,
    pub total_positions: usize,
    pub closed_positions: usize,
    pub active_positions: usize,
    #[serde(with = "rust_decimal::serde::float")]
    pub commissions: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub wins: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub losses: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub win_rate: Decimal,
    pub trading_days: i64,
    pub start_date: String,
    pub end_date: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub max_drawdown: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub max_drawdown_pct: Decimal,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            initial_capital: dec!(0.0),
            pnl: dec!(0.0),
            pnl_pct: dec!(0.0),
            unrealized_pnl: dec!(0.0),
            total_positions: 0,
            closed_positions: 0,
            active_positions: 0,
            commissions: dec!(0.0),
            wins: dec!(0.0),
            losses: dec!(0.0),
            win_rate: dec!(0.0),
            trading_days: 0,
            start_date: "".to_string(),
            end_date: "".to_string(),
            max_drawdown: dec!(0.0),
            max_drawdown_pct: dec!(0.0),
        }
    }
}

pub fn create_stats(backtest: &Backtest) -> Stats {
    let mut wins = dec!(0);
    let mut losses = dec!(0);
    for position in backtest.positions.closed_positions.clone() {
        if position.pnl > dec!(0.0) {
            wins += dec!(1);
        } else {
            losses += dec!(1);
        }
    }
    let mut commissions = backtest.commissions;
    for position in &backtest.positions.active_positions {
        commissions += position.commission;
    }

    let mut win_rate = dec!(0);
    if wins + losses > dec!(0) {
        win_rate = (wins / (wins + losses) * dec!(100.0)).round_dp(2);
    }

    let max_drawdown = calculate_max_drawdown(&backtest.equity).unwrap_or(dec!(0.0));
    let pnl = backtest.equity.last().unwrap() - backtest.params.initial_capital
        + backtest.floating_equity.last().unwrap();
    Stats {
        initial_capital: backtest.params.initial_capital,
        pnl,
        pnl_pct: pnl * dec!(100) / backtest.params.initial_capital,
        unrealized_pnl: *backtest.floating_equity.last().unwrap(),
        total_positions: backtest.positions.active_positions.len()
            + backtest.positions.closed_positions.len(),
        closed_positions: backtest.positions.closed_positions.len(),
        active_positions: backtest.positions.active_positions.len(),
        commissions,
        wins,
        losses,
        win_rate,
        trading_days: backtest
            .ohlc
            .last()
            .unwrap()
            .date
            .signed_duration_since(backtest.ohlc.first().unwrap().date)
            .num_days(),
        start_date: backtest.ohlc.first().unwrap().date.to_string(),
        end_date: backtest.ohlc.last().unwrap().date.to_string(),
        max_drawdown,
        max_drawdown_pct: max_drawdown * dec!(100) / backtest.params.initial_capital,
    }
}
