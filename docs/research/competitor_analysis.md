# Backtesting Libraries Competitor Analysis

*Last updated: January 2026*

---

## Actively Maintained Libraries (2025-2026)

### Open-Source

| Library               | Stars | Language    | Last Update        | Best For                 | Business Model             |
| --------------------- | ----- | ----------- | ------------------ | ------------------------ | -------------------------- |
| **QuantConnect LEAN** | 15.4k | C#/Python   | Active 2025        | Professional multi-asset | Freemium cloud + data fees |
| **Hummingbot**        | 15.5k | Python      | Dec 2025 (v2.11)   | Crypto market making     | Free; enterprise support   |
| **StockSharp**        | 9k    | C#          | Active 2025        | Multi-asset trading      | Free; commercial licenses  |
| **VectorBT**          | 6.4k  | Python      | Dec 2025 (v0.28)   | High-performance, ML     | Free; **VectorBT Pro** (paid)  |
| **HFTBacktest**       | 3.5k  | Rust/Python | Dec 2025 (v0.9.4)  | HFT, market making       | Free (MIT)                 |
| **Finmarketpy**       | 3.7k  | Python      | Mar 2025           | FX, market analysis      | Free                       |
| **pysystemtrade**     | 3.1k  | Python      | Nov 2024 (v1.8.2)  | Systematic futures       | Free; author sells books   |
| **PyBroker**          | 3k    | Python      | Dec 2025 (v1.2.11) | ML/AI trading            | Free; SaaS (TrendNinja.ai) |
| **bt**                | 2.8k  | Python      | Apr 2025 (v1.1.2)  | Portfolio strategies     | Free (MIT)                 |
| **Barter-rs**         | 1.8k  | Rust        | Active 2025        | HFT, Rust developers     | Free (MIT)                 |
| **TradingGym**        | 1.8k  | Python      | Feb 2024           | RL training environment  | Free (MIT)                 |
| **Zipline-reloaded**  | 1.6k  | Python      | Jul 2025 (v3.1.1)  | Quantopian replacement   | Free                       |
| **Lumibot**           | 1.2k  | Python      | Active 2026        | Multi-broker live trading| Free (GPL-3.0); SaaS (BotSpot) |
| **ib_async**          | 1.3k  | Python      | Active 2026        | Interactive Brokers API  | Free (BSD-2)               |
| **OpenAlgo**          | 1.1k  | Python      | Active 2026        | Indian markets (24+ brokers) | Free (AGPL-3.0)        |
| **Basana**            | 806   | Python      | Active 2026        | Async crypto trading     | Free (Apache 2.0)          |

### Slower Updates (Still Maintained)

| Library            | Stars | Language | Last Update | Notes                           |
| ------------------ | ----- | -------- | ----------- | ------------------------------- |
| **Backtesting.py** | 7.7k  | Python   | 2024        | Lightweight, good for beginners |
| **QSTrader**       | 3.3k  | Python   | Jun 2024    | Institutional-style             |

---

## Outdated/Abandoned Libraries (Avoid)

| Library         | Stars | Language | Last Update       | Status                                         |
| --------------- | ----- | -------- | ----------------- | ---------------------------------------------- |
| **Backtrader**  | 20k   | Python   | ~2020             | Abandoned despite popularity                   |
| **PyAlgoTrade** | 4.6k  | Python   | Archived Nov 2023 | Officially deprecated - use **Basana** instead |
| **Fastquant**   | 1.7k  | Python   | ~2021             | Stale, 77 open issues                          |

---

## Commercial Platforms (All Active)

| Platform               | Language         | Business Model             | Pricing                         |
| ---------------------- | ---------------- | -------------------------- | ------------------------------- |
| **QuantConnect**       | Python, C#       | Freemium cloud + data fees | Free tier; $8-80+/mo            |
| **TradingView**        | Pine Script      | Freemium subscriptions     | Free; $14.95-59.95/mo           |
| **AmiBroker**          | AFL              | One-time license           | $299-499                        |
| **MultiCharts**        | PowerLanguage    | License/Subscription       | $99/mo or $1,497 lifetime       |
| **NinjaTrader**        | NinjaScript (C#) | Freemium + Brokerage       | Free; $99/mo or $1,499 lifetime |
| **TradeStation**       | EasyLanguage     | Brokerage commissions      | Varies                          |
| **MetaTrader 4/5**     | MQL4/5           | B2B broker licensing       | Free to traders                 |
| **Bloomberg Terminal** | Python/Excel     | Enterprise subscription    | ~$24,000/year                   |
| **Refinitiv Eikon**    | Python/Excel     | Enterprise subscription    | ~$12,000-22,000/year            |

---

## Business Models Summary

1. **Pure Open-Source** (HFTBacktest, Barter-rs, bt) - Community-driven, no revenue
2. **Freemium** (QuantConnect, TradingView, NinjaTrader) - Free tier + paid features/data
3. **Pro Version Upsell** (VectorBT) - Open-source core + commercial Pro version
4. **One-time License** (AmiBroker, MultiCharts) - Traditional software purchase
5. **Brokerage + Platform** (TradeStation, NinjaTrader) - Make money on trades
6. **B2B Licensing** (MetaTrader) - Free to traders, brokers pay licensing fees
7. **Enterprise SaaS** (Bloomberg, Refinitiv) - High-cost institutional subscriptions
8. **Educational Content** (pysystemtrade) - Free software, sell courses/books

---

## ZenBT Competitive Positioning

### Most Relevant Competitors

| Library         | Why Similar                  | ZenBT Advantage                         |
| --------------- | ---------------------------- | --------------------------------------- |
| **VectorBT**    | Python + performance focus   | Rust core (faster), Grafana integration |
| **HFTBacktest** | Rust core + Python bindings  | Simpler API, less HFT-specific          |
| **PyBroker**    | Python, ML-focused           | Rust performance, lower overhead        |
| **bt**          | Python portfolio backtesting | More flexible order types               |
| **Lumibot**     | Multi-broker, live trading   | Rust performance, Grafana dashboards    |

### ZenBT's Unique Value Proposition

- **Rust performance** with Python ergonomics
- **Not abandoned** like Backtrader (which many still use!)
- **Built-in Grafana dashboards** (unique feature)
- **Simpler than QuantConnect**, faster than pure Python solutions
- Hybrid architecture: Strategy definition in Python, execution in Rust

---

## Links

### Active Open-Source
- QuantConnect LEAN: https://github.com/QuantConnect/Lean
- Hummingbot: https://github.com/hummingbot/hummingbot
- StockSharp: https://github.com/StockSharp/StockSharp
- VectorBT: https://github.com/polakowo/vectorbt
- HFTBacktest: https://github.com/nkaz001/hftbacktest
- Finmarketpy: https://github.com/cuemacro/finmarketpy
- pysystemtrade: https://github.com/robcarver17/pysystemtrade
- PyBroker: https://github.com/edtechre/pybroker
- bt: https://github.com/pmorissette/bt
- Barter-rs: https://github.com/barter-rs/barter-rs
- Zipline-reloaded: https://github.com/stefan-jansen/zipline-reloaded
- Backtesting.py: https://github.com/kernc/backtesting.py
- QSTrader: https://github.com/mhallsmoore/qstrader
- Lumibot: https://github.com/Lumiwealth/lumibot
- ib_async: https://github.com/ib-api-reloaded/ib_async
- OpenAlgo: https://github.com/marketcalls/openalgo
- Basana: https://github.com/gbeced/basana
- TradingGym: https://github.com/Yvictor/TradingGym

### Commercial
- QuantConnect: https://www.quantconnect.com/
- TradingView: https://www.tradingview.com/
- AmiBroker: https://www.amibroker.com/
- MultiCharts: https://www.multicharts.com/
- NinjaTrader: https://www.ninjatrader.com/
- TradeStation: https://www.tradestation.com/
- MetaTrader: https://www.metatrader5.com/
- Bloomberg Terminal: https://www.bloomberg.com/professional/solution/bloomberg-terminal/
- Refinitiv Eikon: https://www.refinitiv.com/en/products/eikon-trading-software
