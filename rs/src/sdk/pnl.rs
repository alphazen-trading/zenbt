use super::instrument::Instrument;
use pyo3::prelude::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

use super::stats::Stats;
use crate::helpers::round_value::round_value;

fn calculate_basis_points(value: Decimal, total: Decimal) -> Decimal {
    (value / total) * dec!(10000.0)
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PNL {
    instruments: Vec<Instrument>,
    use_ask_list: Vec<bool>,
    quantities: Vec<Decimal>,
    prices: Vec<Decimal>,
    commission: Decimal,
    verbose: bool,
    stats: Stats,
}
#[pymethods]
impl PNL {
    #[new]
    fn new(
        instruments: Vec<Instrument>,
        use_ask_list: Vec<bool>,
        commission: Decimal,
        verbose: bool,
    ) -> Self {
        Self {
            instruments,
            use_ask_list,
            quantities: vec![],
            prices: vec![],
            commission,
            verbose,
            stats: Stats::default(),
        }
    }

    #[getter]
    fn pnl_bps(&self) -> Decimal {
        self.stats.pnl_bps
    }

    #[getter]
    fn pnl(&self) -> Decimal {
        self.stats.pnl
    }

    #[getter]
    fn instruments(&self) -> Vec<Instrument> {
        self.instruments.clone()
    }

    #[getter]
    fn quantities(&self) -> Vec<Decimal> {
        self.stats.quantities.clone()
    }

    #[getter]
    fn prices(&self) -> Vec<Decimal> {
        self.stats.prices.clone()
    }

    pub fn update_instrument_bbo(&mut self, instrument: Instrument) {
        for _instrument in self.instruments.iter_mut() {
            if _instrument.id == instrument.id {
                _instrument.on_new_bbo(instrument.bbo.clone());
            }
        }
    }

    pub fn find_max_capacity(&mut self) -> Decimal {
        let min_usd_qty = dec!(10000000);
        self.reset(min_usd_qty);
        let excess_ratio = self.calculate(min_usd_qty);
        min_usd_qty / excess_ratio
    }

    fn reset(&mut self, min_usd_qty: Decimal) {
        self.prices = vec![];
        self.quantities = vec![min_usd_qty];
    }

    fn find_pnl(&mut self, min_usd_qty: Decimal) {
        self.reset(min_usd_qty);

        let mut excess_ratio = self.calculate(min_usd_qty);
        if excess_ratio > dec!(1.0) {
            let new_min_usd_qty = min_usd_qty / excess_ratio;
            self.reset(new_min_usd_qty);
            if self.verbose {
                println!("");
                println!("");
                println!("Excess ratio exceeded.\nAdjust Min qty to {}", min_usd_qty);
                println!();
            }
            excess_ratio = self.calculate(new_min_usd_qty);
        }
        if excess_ratio != dec!(-1) {
            self.calculate_pnl();
        }
    }

    fn calculate_pnl(&mut self) -> Decimal {
        let quantities = &self.quantities;
        let initial_usd = quantities[0];

        let commissions = (quantities[0] * self.commission).powd(dec!(3.0));
        let last_quantity = self.quantities[self.quantities.len() - 1];
        let last_price = self.prices[self.prices.len() - 1];

        let final_usd = last_quantity * last_price - commissions;
        let pnl = final_usd - initial_usd;
        let pnl_bps = calculate_basis_points(pnl, initial_usd);
        self.stats.quantities = quantities[1..].to_vec();
        self.stats.prices = self.prices.clone();

        self.stats.commissions = commissions;
        self.stats.initial_usd = initial_usd;
        self.stats.final_usd = final_usd;
        self.stats.pnl = pnl;
        self.stats.pnl_bps = pnl_bps;
        pnl_bps
    }

    fn calculate(&mut self, min_usd_qty: Decimal) -> Decimal {
        let mut carried_coins = min_usd_qty;
        let mut excess_ratio = dec!(1.0);

        let mut messages = Vec::new();

        for (j, instrument) in self.instruments.iter().enumerate() {
            // instrument.bbo.print();
            let use_ask = self.use_ask_list[j];

            let curr_price = if use_ask {
                instrument.bbo.bid_price
            } else {
                instrument.bbo.ask_price
            };

            let curr_qty = if use_ask {
                instrument.bbo.bid_size
            } else {
                instrument.bbo.ask_size
            };

            // let mut curr_max_qty_available = curr_qty * curr_price;
            // if use_ask {
            //     curr_max_qty_available = curr_qty;
            // }
            let curr_max_qty_available = curr_qty;
            let _carried_coins = carried_coins;

            let rounded_carried_coins = round_value(carried_coins, instrument.min_order);
            if rounded_carried_coins > curr_max_qty_available {
                excess_ratio = excess_ratio.max(rounded_carried_coins / curr_max_qty_available);
                if self.verbose {
                    println!(
                        "Warning to sell more than the available by {}",
                        excess_ratio
                    );
                }
            }

            if rounded_carried_coins == dec!(0.0) {
                return dec!(-1);
            }

            if use_ask {
                carried_coins = rounded_carried_coins * curr_price;
            } else {
                carried_coins = rounded_carried_coins / curr_price;
            }

            self.prices.push(curr_price);
            self.quantities.push(rounded_carried_coins);

            if self.verbose {
                let mut target_coin = &instrument.contract.exchange_base_underlying;
                let mut result_coin = &instrument.contract.exchange_counter_underlying;
                if !use_ask {
                    target_coin = &instrument.contract.exchange_counter_underlying;
                    result_coin = &instrument.contract.exchange_base_underlying;
                }

                messages.push(format!(
                    "========== {} @ {} -- {} {} -- {}",
                    target_coin, curr_price, instrument.contract.code, use_ask, min_usd_qty
                ));
                messages.push(format!(
                    "I can sell {} of {}",
                    curr_max_qty_available, target_coin
                ));
                messages.push(format!(
                    "I have {} {} but can only sell {}",
                    _carried_coins, target_coin, rounded_carried_coins
                ));

                messages.push(format!(
                    "I will sell {} of {} to get {} {}",
                    rounded_carried_coins, target_coin, carried_coins, result_coin
                ));
            }
        }
        self.quantities[1] = self.quantities[2];
        if self.verbose {
            for message in messages {
                println!("{}", message);
            }
            println!("Quantities are: {:?}", self.quantities);
        }
        excess_ratio
    }
}

impl Default for PNL {
    fn default() -> Self {
        Self {
            instruments: [].to_vec(),
            use_ask_list: [].to_vec(),
            quantities: [].to_vec(),
            prices: [].to_vec(),
            commission: dec!(0.0018),
            verbose: false,
            stats: Stats::default(),
        }
    }
}
