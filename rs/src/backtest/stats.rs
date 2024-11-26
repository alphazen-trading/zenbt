use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Stats of the backtest
///
/// Attributes:
///     initial_capital (decimal): The initial capital of the backtest
///     pnl (decimal): The profit and loss of the backtest
///     pnl_pct (decimal): The profit and loss percentage of the backtest
///     unrealized_pnl (decimal): The unrealized profit and loss of the backtest
///     total_positions (int): The total number of positions in the backtest
///     closed_positions (int): The number of closed positions in the backtest
///     active_positions (int): The number of active positions in the backtest
///     commissions (decimal): The total commissions of the backtest
///     wins (decimal): The number of wins in the backtest
///     losses (decimal): The number of losses in the backtest
///     win_rate (str): The win rate of the backtest
///     trading_days (int): The number of trading days in the backtest
///     start_date (str): The start date of the backtest
///     end_date (str): The end date of the backtest
///     max_drawdown (decimal): The maximum drawdown of the backtest
///     max_drawdown_pct (decimal): The maximum drawdown percentage of the backtest
///
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
    pub win_rate: String,
    pub trading_days: i64,
    pub start_date: String,
    pub end_date: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub max_drawdown: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub max_drawdown_pct: Decimal,
}

impl ToPyObject for Stats {
    fn to_object(&self, py: Python) -> PyObject {
        // Create a new Python dictionary
        let dict = PyDict::new_bound(py);

        // Convert and insert fields into the dictionary
        dict.set_item("initial_capital", self.initial_capital)
            .unwrap();
        dict.set_item("pnl", self.pnl).unwrap();
        dict.set_item("pnl_pct", self.pnl_pct).unwrap();
        dict.set_item("unrealized_pnl", self.unrealized_pnl)
            .unwrap();
        dict.set_item("total_positions", self.total_positions)
            .unwrap(); // usize
        dict.set_item("closed_positions", self.closed_positions)
            .unwrap(); // usize
        dict.set_item("active_positions", self.active_positions)
            .unwrap(); // usize
        dict.set_item("commissions", self.commissions).unwrap();
        dict.set_item("wins", self.wins).unwrap();
        dict.set_item("losses", self.losses).unwrap();
        dict.set_item("win_rate", self.win_rate.clone()).unwrap();
        dict.set_item("trading_days", self.trading_days).unwrap(); // i64
        dict.set_item("start_date", self.start_date.clone())
            .unwrap(); // String
        dict.set_item("end_date", self.end_date.clone()).unwrap(); // String
        dict.set_item("max_drawdown", self.max_drawdown).unwrap();
        dict.set_item("max_drawdown_pct", self.max_drawdown_pct)
            .unwrap();

        // Return the Python dictionary as a PyObject
        dict.to_object(py)
    }
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
            win_rate: "0%".to_string(),
            trading_days: 0,
            start_date: String::new(),
            end_date: String::new(),
            max_drawdown: dec!(0.0),
            max_drawdown_pct: dec!(0.0),
        }
    }
}
