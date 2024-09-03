use pyo3::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Stats {
    pub quantities: Vec<Decimal>,
    pub prices: Vec<Decimal>,
    pub commissions: Decimal,
    pub initial_usd: Decimal,
    pub final_usd: Decimal,
    pub pnl: Decimal,
    pub pnl_bps: Decimal,
}
#[pymethods]
impl Stats {
    #[new]
    fn new(
        quantities: Vec<Decimal>,
        prices: Vec<Decimal>,
        commissions: Decimal,
        initial_usd: Decimal,
        final_usd: Decimal,
        pnl: Decimal,
        pnl_bps: Decimal,
    ) -> Self {
        Self {
            quantities,
            prices,
            commissions,
            initial_usd,
            final_usd,
            pnl,
            pnl_bps,
        }
    }

    #[getter]
    fn quantities(&self) -> Vec<Decimal> {
        self.quantities.clone()
    }

    #[getter]
    fn prices(&self) -> Vec<Decimal> {
        self.prices.clone()
    }

    #[getter]
    fn commissions(&self) -> Decimal {
        self.commissions
    }

    #[getter]
    fn initial_usd(&self) -> Decimal {
        self.initial_usd
    }

    #[getter]
    fn final_usd(&self) -> Decimal {
        self.final_usd
    }

    #[getter]
    fn pnl(&self) -> Decimal {
        self.pnl
    }

    pub fn print(&self) {
        println!("Quantities: {:?}", self.quantities);
        println!("Prices: {:?}", self.prices);
        println!("Commissions: {:?}", self.commissions);
        println!("Initial USD: {:?}", self.initial_usd);
        println!("Final USD: {:?}", self.final_usd);
        println!("PnL: {:?}", self.pnl);
        println!("PnL BPS: {:?}", self.pnl_bps);
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            quantities: vec![],
            prices: vec![],
            commissions: dec!(0.0),
            initial_usd: dec!(0.0),
            final_usd: dec!(0.0),
            pnl: dec!(0.0),
            pnl_bps: dec!(0.0),
        }
    }
}
