use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

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
