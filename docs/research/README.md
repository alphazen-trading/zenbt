# ZenBT Research Documents

*Last updated: January 2026*

This folder contains research on the trading software ecosystem, business models, and monetization strategies.

## Documents

| Document | Description | Key Findings |
|----------|-------------|--------------|
| [github_comprehensive_research.md](github_comprehensive_research.md) | **NEW** Exhaustive GitHub research | 2,700+ projects, Rust+Python hybrids, emerging frameworks |
| [broker_data_comparison.md](broker_data_comparison.md) | **NEW** IB vs Alpaca data/API comparison | Auth, historical data limits, pricing |
| [ccxt_business_model.md](ccxt_business_model.md) | CCXT's broker program analysis | 28 exchanges with broker codes, 6 DEXes |
| [broker_program_research.md](broker_program_research.md) | Competitor broker code analysis | Hummingbot, Freqtrade, NautilusTrader, Jesse |
| [defi_referral_programs.md](defi_referral_programs.md) | DeFi builder/referral programs | Hyperliquid, Jupiter, 0x, dYdX |
| [competitor_analysis.md](competitor_analysis.md) | Backtesting libraries comparison | VectorBT, HFTBacktest, PyBroker, etc. |
| [execution_systems_analysis.md](execution_systems_analysis.md) | Live trading platforms | Freqtrade, NautilusTrader, Hummingbot, Jesse |
| [linear_tickets.md](linear_tickets.md) | ZenBT improvement tickets | Security, testing, architecture tasks |

## Quick Reference

### CCXT Broker Codes (28 Total)

**CeFi (22 exchanges):**
- Binance, Bybit, OKX, BitMart, CoinEx, BingX, Bitget, HTX, KuCoin, MEXC, Crypto.com, Phemex, WhiteBIT, WOO X, BitMEX, Coinbase, Coinbase Intl, BloFin, CoinCatch, HashKey, BitTrade, Tokocrypto

**DeFi (6 exchanges):**
- Hyperliquid (Builder: `0x6530512A...`)
- Apex (`6956`)
- WOOFi Pro (`CCXT`)
- ModeTrade (`CCXTMODE`)
- Paradex (`CCXT`)
- Derive (`0x0ad42b8e...`)

### Commission Rates

| Type | Platform | Rate |
|------|----------|------|
| CeFi Broker | Binance, OKX, Bybit | 20-50% of fees |
| CeFi Broker | KuCoin, HTX, Gate.io | Up to 65-70% of fees |
| DeFi Builder | Hyperliquid | Up to 0.1% of trade value |
| DeFi Integrator | Jupiter, 0x | Flexible platform fees |

### Competitor Broker Codes

| Platform | Exchanges | Key Codes |
|----------|-----------|-----------|
| **CCXT** | 28 | Binance: `x-TKT5PX2F`, OKX: `6b9ad766b55dBCDE` |
| **Hummingbot** | 5 (9 connectors) | Binance: `x-MG43PCSN`, OKX: `93027a12dac34fBC` |
| **NautilusTrader** | 2 | Bybit: `Qy000878`, OKX: `a535cbe8d0c8BCDE` |
| **Freqtrade** | 1 | OKX: `ffb5405ad327SUDE` |

## ZenBT Monetization Options

If ZenBT adds live trading:

1. **Use CCXT** - Low effort, $0 revenue (CCXT gets commissions)
2. **Override CCXT codes** - Medium effort, replace with own broker IDs
3. **Native implementations** - High effort, full commission control
4. **DeFi builder programs** - Apply to Hyperliquid, Jupiter, etc.

### Priority Partnerships

| Priority | Exchange | Type | Max Commission |
|----------|----------|------|----------------|
| 1 | KuCoin | CeFi | 70% |
| 2 | Hyperliquid | DeFi | 0.1% per trade |
| 3 | Binance | CeFi | 50% |
| 4 | Jupiter | DeFi | Flexible |

---

## GitHub Research Summary (Jan 2026)

### Projects Discovered: 2,700+

| Category | Count | Top Projects |
|----------|-------|--------------|
| **Rust+Python Hybrid** | 28 | nautilus_trader (17k), hftbacktest (3.5k), pyrust-bt (276) |
| **Pure Rust** | 50+ | barter-rs (1.8k), RustQuant (1.6k), ta-rs (819), yata (385) |
| **Emerging Frameworks** | 15+ | SimTradeLab (146), PyneCore (81), QTradeX (55) |
| **DeFi Backtesting** | 22 | demeter (81), uniswap-v3-backtest (60) |
| **Solana Trading** | 1,500+ | warp-id/solana-trading-bot (2.3k) |
| **Hyperliquid** | 269 | passivbot (1.8k), hyperliquid-trading-bot (225) |
| **Grafana Trading** | 10 | freqtrade-dashboard (47) - **all monitoring, none for backtest viz** |

### Major Competitors (Previously Missing)

| Project | Stars | Key Features |
|---------|-------|--------------|
| [Lumibot](https://github.com/Lumiwealth/lumibot) | 1.2k | Multi-broker (Alpaca, IB, Tradier), Options Greeks, SaaS platform |
| [ib_async](https://github.com/ib-api-reloaded/ib_async) | 1.3k | Interactive Brokers API (replaces ib_insync) |
| [OpenAlgo](https://github.com/marketcalls/openalgo) | 1.1k | 24+ Indian broker integrations, unified API |
| [Basana](https://github.com/gbeced/basana) | 806 | Async crypto trading, event-driven |
| [TradingGym](https://github.com/Yvictor/TradingGym) | 1.8k | OpenAI Gym for trading, RL training |

### ZenBT's Unique Position

**No other project combines all three:**
1. Rust-powered backtesting core
2. Python API via PyO3
3. Built-in Grafana dashboards

### Key Emerging Projects to Watch

| Project | Stars | Why Notable |
|---------|-------|-------------|
| [pyrust-bt](https://github.com/QuantML-C/pyrust-bt) | 276 | 250x faster, similar architecture to ZenBT |
| [wingfoil](https://github.com/wingfoil-io/wingfoil) | 104 | DAG-based stream processing |
| [SimTradeLab](https://github.com/kay-ou/SimTradeLab) | 146 | Interesting caching patterns |
| [PyneCore](https://github.com/PyneSys/pynecore) | 81 | Pine Script in Python via AST |
| [microalpha](https://github.com/MateoBodon/microalpha) | 1 | Walk-forward + factor analysis |
| [Meridian](https://github.com/sukesan7/meridian) | 0 | Determinism verification in CI |

See [github_comprehensive_research.md](github_comprehensive_research.md) for full details.
