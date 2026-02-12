# Comprehensive GitHub Research: Trading & Backtesting Ecosystem

*Last updated: January 2026*

This document contains an exhaustive analysis of all trading, backtesting, and quant libraries on GitHub, including smaller and emerging projects that may not be well-known.

---

## Executive Summary

| Category | Total Projects Found | Top 3 by Stars |
|----------|---------------------|----------------|
| **Rust+Python Hybrid** | 28 | NautilusTrader (17k), HFTBacktest (3.5k), pyrust-bt (276) |
| **Pure Rust** | 50+ | NautilusTrader (17k), Barter-rs (1.8k), RustQuant (1.6k) |
| **Emerging Frameworks** | 15+ | SimTradeLab (146), PyneCore (81), QTradeX (55) |
| **High-Performance Python** | 30+ | VectorBT (6.4k), backtesting.py (7.7k), pyqstrat (369) |
| **Crypto-Specific** | 2,600+ | Freqtrade (45.7k), warp-id/solana-trading-bot (2.3k) |
| **Grafana/Dashboard** | 10 | freqtrade-dashboard (47), bt-visualizer (25) |

**Key Finding:** ZenBT's combination of Rust core + Python bindings + Grafana dashboards is **unique** - no other project offers all three.

---

## Part 0: Major Competitors (Previously Missed)

These are significant projects that were not in the original competitor_analysis.md:

### Lumibot (1.2k stars) - Direct Competitor
**URL:** https://github.com/Lumiwealth/lumibot

| Attribute | Details |
|-----------|---------|
| Stars | 1.2k |
| License | GPL-3.0 |
| Last Update | Active (Jan 2026) |

**Key Features:**
- Backtesting and live trading with **same code**
- Multi-asset: Stocks, Options, Crypto, Futures, FOREX
- Built-in data sources: Yahoo, ThetaData, Polygon, Alpaca, Tradier
- Interactive HTML reports with charts
- Options Greeks support

**Supported Brokers:**
- Alpaca (stocks, crypto)
- Interactive Brokers
- Tradier
- Coinbase, Kraken (crypto)

**Business Model:**
- Open source (GPL-3.0)
- **BotSpot.trade** - Paid SaaS platform
- **AI Bootcamp** - Paid educational courses
- **ThetaData partnership** - Referral revenue

**Relevance to ZenBT:** Direct competitor. Python-only. ZenBT's Rust core is a performance differentiator.

---

### ib_async (1.3k stars) - IB API Successor
**URL:** https://github.com/ib-api-reloaded/ib_async

Replaces the archived ib_insync (3.2k stars). Full async/await support for Interactive Brokers.

---

### Basana (806 stars) - Async Crypto Trading
**URL:** https://github.com/gbeced/basana

Event-driven async architecture. Supports Binance, Bitstamp. Good reference for async patterns.

---

### OpenAlgo (1.1k stars) - Indian Markets Platform
**URL:** https://github.com/marketcalls/openalgo

**24+ Indian broker integrations** with unified API. REST API layer, WebSocket streaming with ZeroMQ. MCP Server for AI agents. Excellent multi-broker integration pattern.

---

### algo-trader (860 stars) - Pipeline Architecture
**URL:** https://github.com/idanya/algo-trader

JSON-serializable pipeline architecture. Processors, Sources, Terminators pattern. Worth studying.

---

### TradingGym (1.8k stars) - RL Environment
**URL:** https://github.com/Yvictor/TradingGym

OpenAI Gym-style interface for trading. Reinforcement learning training environment.

---

### High-Frequency Trading Resources

| Repository | Stars | Description |
|------------|-------|-------------|
| [jamesmawm/HFT-Model-with-IB](https://github.com/jamesmawm/High-Frequency-Trading-Model-with-IB) | 2.8k | HFT with pairs trading & mean-reversion |

---

## Part 1: Rust + Python Hybrid Libraries

### Tier 1: Major Projects (1000+ Stars)

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [nautechsystems/nautilus_trader](https://github.com/nautechsystems/nautilus_trader) | 17k | Jan 2026 | Production-grade, 15+ exchanges, event-driven, nanosecond resolution |
| [nkaz001/hftbacktest](https://github.com/nkaz001/hftbacktest) | 3.5k | Dec 2025 | HFT/market making, queue position simulation, LOB reconstruction |

### Tier 2: Growing Projects (100-1000 Stars)

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [QuantML-C/pyrust-bt](https://github.com/QuantML-C/pyrust-bt) | 276 | Nov 2025 | 250x faster, cross-sectional factors, FastAPI+Streamlit, DuckDB |
| [wingfoil-io/wingfoil](https://github.com/wingfoil-io/wingfoil) | 104 | Jan 2026 | DAG-based stream processing, ultra-low latency, Tokio async |

### Tier 3: Emerging Projects (10-100 Stars)

| Repository | Stars | Last Update | Description |
|------------|-------|-------------|-------------|
| [nersent/qpace](https://github.com/nersent/qpace) | 23 | Nov 2025 | Quant SDK for Python/JS, Rust core |
| [pbeets/rithmic-rs](https://github.com/pbeets/rithmic-rs) | 16 | Dec 2025 | Rust connector for Rithmic API (futures) |
| [jerryinyang/rustybt](https://github.com/jerryinyang/rustybt) | 5 | Dec 2025 | Rust speed + Python ease, quant researchers |
| [pinsky-three/greenrock](https://github.com/pinsky-three/greenrock) | 4 | Nov 2025 | AI-powered quant platform |
| [Hawk-Center/hawk-backtester](https://github.com/Hawk-Center/hawk-backtester) | 3 | Nov 2025 | Portfolio backtesting with Python bindings |

### Tier 4: Very New Projects (0-10 Stars)

| Repository | Stars | Last Update | Description |
|------------|-------|-------------|-------------|
| [LenWilliamson/chapaty](https://github.com/LenWilliamson/chapaty) | 0 | Jan 2026 | Backtesting + RL agents (Gym-compatible) |
| [SamoraDC/RustAlgorithmTrading](https://github.com/SamoraDC/RustAlgorithmTrading) | 0 | Dec 2025 | Python research + Rust execution |
| [yuno-research/solana_copy_trading](https://github.com/yuno-research/solana_copy_trading) | 1 | Jan 2026 | Solana copy trading + backtesting |
| [i7xh/weight_backtest_pyo3](https://github.com/i7xh/weight_backtest_pyo3) | 0 | Jun 2025 | Weight-based backtesting with PyO3 |
| [Yvictor/polars_backtest_extension](https://github.com/Yvictor/polars_backtest_extension) | 0 | Jan 2026 | Rust Polars extension for backtesting |

---

## Part 2: Pure Rust Libraries

### Trading Platforms & Frameworks

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [barter-rs/barter-rs](https://github.com/barter-rs/barter-rs) | 1.8k | Jan 2026 | Event-driven, modular (data/execution/instrument), MIT |
| [avhz/RustQuant](https://github.com/avhz/RustQuant) | 1.6k | Sep 2025 | Derivatives, stochastics, autodiff, ML |
| [Lqz13Th/extrema_infra](https://github.com/Lqz13Th/extrema_infra) | 83 | Jan 2026 | HList static dispatch, ZeroMQ ML, lock-free |
| [jensnesten/rust_bt](https://github.com/jensnesten/rust_bt) | 45 | Dec 2025 | High-performance, low-latency backtesting |

### Technical Analysis Libraries

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [greyblake/ta-rs](https://github.com/greyblake/ta-rs) | 819 | Jul 2024 | Classic TA indicators, trait-based |
| [amv-dev/yata](https://github.com/amv-dev/yata) | 385 | Sep 2024 | Fastest (3-7ns per indicator), 15+ MAs |
| [gregyjames/ZenithTA](https://github.com/gregyjames/ZenithTA) | 219 | May 2025 | Rust + NumPy C API, Python bindings |
| [chironmind/RustTI](https://github.com/chironmind/RustTI) | 32 | Nov 2025 | Technical indicators package |
| [uyplayer/rusty-talib](https://github.com/uyplayer/rusty-talib) | 15 | Dec 2023 | Pure Rust TA-Lib implementation |

### Order Book & Matching Engines

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [llc-993/matching-core](https://github.com/llc-993/matching-core) | 47 | Jan 2026 | 7M+ TPS, <1μs latency, LMAX Disruptor pattern |

### Market Data

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [databento/dbn](https://github.com/databento/dbn) | 145 | Dec 2025 | Fast binary encoding for market data |
| [borsaorg/borsa](https://github.com/borsaorg/borsa) | 16 | Nov 2025 | Pluggable market data orchestrator |
| [joaquinbejar/DXlink](https://github.com/joaquinbejar/DXlink) | 8 | Dec 2025 | tastytrade WebSocket client |
| [joaquinbejar/tradier](https://github.com/joaquinbejar/tradier) | 6 | Nov 2025 | Tradier broker API |

### Quantitative Finance

| Repository | Stars | Last Update | Key Features |
|------------|-------|-------------|--------------|
| [carlobortolan/quantrs](https://github.com/carlobortolan/quantrs) | 13 | Jan 2026 | Derivatives & option pricing |
| [siddharthqs/RustyQLib](https://github.com/siddharthqs/RustyQLib) | 19 | Mar 2025 | QuantLib-inspired |
| [compascafe/zzignal](https://github.com/compascafe/zzignal) | 2 | Nov 2025 | Python bindings for quant finance |

---

## Part 3: Emerging Backtesting Frameworks (Not Mainstream Yet)

These are newer projects worth watching:

### SimTradeLab (146 stars) - Chinese Markets Focus
**URL:** https://github.com/kay-ou/SimTradeLab
- PTrade-compatible, 20-30x faster than PTrade
- AST analysis for smart data loading
- Multi-level LRU caching
- FIFO batch position management with dividend tax

### PyneCore (81 stars) - Pine Script in Python
**URL:** https://github.com/PyneSys/pynecore
- Native Pine Script semantics via AST transformation
- Series & Persistent variables
- Can compile Pine Script to Python
- Zero dependencies

### QTradeX-Algo-Trading-SDK (55 stars) - AI Optimization
**URL:** https://github.com/squidKid-deluxe/QTradeX-Algo-Trading-SDK
- 50+ backtests/sec on Raspberry Pi
- QPSO & LSGA optimization engines
- CCXT integration (100+ exchanges)
- Tulip indicators

### AlphaFlow (5 stars) - Clean Architecture
**URL:** https://github.com/brandonschabell/alphaflow
- Event-driven with pub-sub (EventBus)
- Plans for Rust integration
- Professional analytics (Sharpe, Sortino)

### microalpha (1 star) - Research-Grade
**URL:** https://github.com/MateoBodon/microalpha
- Walk-forward cross-validation
- HAC Sharpe estimates
- Factor regression (FF3/FF5)
- 78% test coverage

### Meridian (0 stars) - Deterministic Engine
**URL:** https://github.com/sukesan7/meridian
- Determinism verification in CI
- Regime-adaptive slippage
- Session-aware execution
- Monte Carlo risk assessment

---

## Part 4: High-Performance Python Libraries

### Vectorized Backtesting

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [polakowo/vectorbt](https://github.com/polakowo/vectorbt) | 6.4k | Numba-accelerated, 10k+ params, Plotly 6 |
| [ematvey/pybacktest](https://github.com/ematvey/pybacktest) | 817 | Vectorized pandas, compact |
| [keithorange/SuperFastBacktest](https://github.com/keithorange/SuperFastBacktest) | 2 | Vectorized + parallelized |

### Numba-Accelerated

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [cyclux/tradeforce](https://github.com/cyclux/tradeforce) | 10 | Hyperparameter optimization |
| [JoaoZati/backtesting_walkfoward](https://github.com/JoaoZati/backtesting_walkfoward) | 2 | Walk-forward enabled |
| [MARESH001/NUMBA-GPU_accelrated_backtesting](https://github.com/MARESH001/NUMBA-GPU_accelrated_backtesting) | 0 | **GPU-accelerated** with CUDA |

### Polars-Based

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [IsaacCheng9/quant-trading-strategy-backtester](https://github.com/IsaacCheng9/quant-trading-strategy-backtester) | 27 | Interactive Plotly dashboard |
| [rwspielman/ffn-polars](https://github.com/rwspielman/ffn-polars) | 4 | Polars reimplementation of ffn |
| [shaojintian/crypto_backtesting](https://github.com/shaojintian/crypto_backtesting) | 3 | Polars + Parquet |

### Parallel Processing

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [pawelkn/btester](https://github.com/pawelkn/btester) | 21 | Multi-asset, parallel optimization |
| [AlgoETS/BatchBacktesting](https://github.com/AlgoETS/BatchBacktesting) | 1 | Batch backtests in parallel |

---

## Part 5: Crypto-Specific Tools

### DeFi Backtesting

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [zelos-alpha/demeter](https://github.com/zelos-alpha/demeter) | 81 | Uniswap, GMX, AAVE LP/lending/options |
| [PFund-Software-Ltd/pfund](https://github.com/PFund-Software-Ltd/pfund) | 62 | Backtest -> Train -> Trade -> Monitor |
| [DefiLab-xyz/uniswap-v3-backtest](https://github.com/DefiLab-xyz/uniswap-v3-backtest) | 60 | Fast Uniswap V3 LP (JavaScript) |
| [smolquants/backtest-ape](https://github.com/smolquants/backtest-ape) | 27 | DeFi + Monte Carlo |

### Funding Rate Arbitrage

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [aoki-h-jp/funding-rate-arbitrage](https://github.com/aoki-h-jp/funding-rate-arbitrage) | 282 | Binance, Gate.io, Bybit |
| [50shadesofgwei/funding-rate-arbitrage](https://github.com/50shadesofgwei/funding-rate-arbitrage) | 161 | Delta-neutral arb searcher |
| [ksmit323/funding-rate-arbitrage](https://github.com/ksmit323/funding-rate-arbitrage) | 75 | Hackathon winner |
| [supervik/funding-rate-arbitrage-scanner](https://github.com/supervik/funding-rate-arbitrage-scanner) | 27 | Perp-Perp and Perp-Spot scanner |

### Solana Trading Bots

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [warp-id/solana-trading-bot](https://github.com/warp-id/solana-trading-bot) | 2.3k | Token sniping (TypeScript) |
| [i3visio/solana-mev-bot](https://github.com/i3visio/solana-mev-bot) | 1.2k | MEV bot, bundler, sniper (Rust) |
| [radioman/Auto-solana-trading-bot](https://github.com/radioman/Auto-solana-trading-bot) | 1k | PumpFun, PumpSwap, Raydium via gRPC |
| [outsmartchad/solana-trading-cli](https://github.com/outsmartchad/solana-trading-cli) | 564 | CLI, gRPC, Jito, Nozomi |
| [henrytirla/Solana-Trading-Bot](https://github.com/henrytirla/Solana-Trading-Bot) | 291 | Raydium & Pump.fun (Python) |

### Hyperliquid Ecosystem

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [enarjord/passivbot](https://github.com/enarjord/passivbot) | 1.8k | Multi-exchange including Hyperliquid |
| [kallie45s/hyperliquid-trading-bot](https://github.com/kallie45s/hyperliquid-trading-bot) | 225 | Hyperliquid trading (Python) |
| [kallie45s/hyperliquid-arbitrage-bot](https://github.com/kallie45s/hyperliquid-arbitrage-bot) | 223 | Hyperliquid arbitrage (TypeScript) |
| [Jackhuang166/hyberliquid-arbitrage](https://github.com/Jackhuang166/hyberliquid-arbitrage) | 158 | Bybit-Hyperliquid price gap (Rust) |
| [0xNoSystem/hyperliquid_rust_bot](https://github.com/0xNoSystem/hyperliquid_rust_bot) | 21 | Automated trading (Rust) |

### Perpetual Backtesting

| Repository | Stars | Description |
|------------|-------|-------------|
| [breaded-xyz/alphavec](https://github.com/breaded-xyz/alphavec) | 1 | Fast minimalist perp backtesting |
| [yllvar/hyperliquid-ai-backtest](https://github.com/yllvar/hyperliquid-ai-backtest) | 0 | Rust + LLM optimization |
| [CHANGGELY/liangzhi-backtest](https://github.com/CHANGGELY/liangzhi-backtest) | 3 | Market-making + ATR grid |

---

## Part 6: Visualization & Dashboard Tools

### Grafana-Based (Similar to ZenBT)

| Repository | Stars | Stack | Focus |
|------------|-------|-------|-------|
| [thraizz/freqtrade-dashboard](https://github.com/thraizz/freqtrade-dashboard) | 47 | Grafana + Prometheus | Freqtrade bot monitoring |
| [questdb/questdb-trading-data-demo](https://github.com/questdb/questdb-trading-data-demo) | 7 | Grafana + QuestDB | Real-time tick data |
| [myownipgit/freqtrade-monitoring-stack](https://github.com/myownipgit/freqtrade-monitoring-stack) | 3 | Grafana + Prometheus + Supabase | Crypto bot monitoring |
| [Chaitanya-cpc/grafana_trading_dash](https://github.com/Chaitanya-cpc/grafana_trading_dash) | 0 | Grafana + InfluxDB | Zerodha trading |

### Other Visualization

| Repository | Stars | Key Features |
|------------|-------|--------------|
| [kernc/backtesting.py](https://github.com/kernc/backtesting.py) | 7.7k | Interactive Bokeh charts |
| [ranaroussi/quantstats](https://github.com/ranaroussi/quantstats) | 6.5k | HTML tearsheets, 50+ metrics |
| [matplotlib/mplfinance](https://github.com/matplotlib/mplfinance) | 4.3k | OHLC/Candlestick charts |
| [Finance-Insight-Lab/bt-visualizer](https://github.com/Finance-Insight-Lab/bt-visualizer) | 25 | TradingView charts in JupyterLab |

---

## Part 7: ZenBT Competitive Analysis

### Expanded Competitor Comparison

| Feature | ZenBT | NautilusTrader | HFTBacktest | Lumibot | VectorBT | PyBroker |
|---------|-------|----------------|-------------|---------|----------|----------|
| Rust Core | Yes | Yes | Yes | No | No (Numba) | No (Numba) |
| Python API | Yes (PyO3) | Yes | Yes | Native | Native | Native |
| Grafana Integration | **Yes (Unique)** | No | No | No | No | No |
| Multi-Broker | No | Yes (15+) | Limited | Yes (5+) | No | Limited |
| Live Trading | No (planned) | Yes | Yes | Yes | No | No |
| ML Integration | No | Yes | No | Basic | Yes | **Strong** |
| Options Support | No | Yes | No | Yes | No | No |
| Business Model | - | Enterprise | Open | SaaS+Education | Pro version | SaaS |

### What Makes ZenBT Unique

| Feature | ZenBT | NautilusTrader | HFTBacktest | VectorBT | Barter-rs |
|---------|-------|----------------|-------------|----------|-----------|
| Rust Core | Yes | Yes | Yes | No (Numba) | Yes |
| Python API | Yes (PyO3) | Yes | Yes | Native | No |
| Grafana Integration | **Yes (Unique)** | No | No | No | No |
| ECharts Support | **Yes (Unique)** | No | No | No | No |
| Simple API | Yes | Complex | Medium | Simple | Medium |
| HFT Focus | No | Yes | Yes | No | Yes |
| Live Trading | No (planned) | Yes | Yes | No | Yes |

### ZenBT's True Differentiators

1. **Grafana Integration** - No other backtesting library has native Grafana support
2. **ECharts Panel** - Custom visualizations beyond standard charts
3. **Simplicity** - Simpler than NautilusTrader/HFTBacktest
4. **Rust+Python Balance** - Performance without complexity

### Competitive Gaps to Consider

| Gap | Projects Addressing It | ZenBT Opportunity |
|-----|----------------------|-------------------|
| Walk-Forward Validation | microalpha, Meridian, PyBroker | Built-in walk-forward |
| Determinism Verification | Meridian | CI gate for reproducibility |
| Pine Script Compat | PyneCore | Import Pine strategies? |
| Factor Analysis | microalpha | FF3/FF5 regression |
| GPU Acceleration | NUMBA-GPU_accelrated | Optional GPU mode |
| Multi-Broker Support | Lumibot, OpenAlgo, NautilusTrader | Unified broker API |
| ML Integration | PyBroker, VectorBT | Walkforward analysis |
| Options Support | Lumibot, NautilusTrader | Greeks calculation |
| SaaS/Cloud Platform | Lumibot (BotSpot), QuantConnect | Hosted version |

### Business Model Patterns in Ecosystem

| Model | Examples | Notes |
|-------|----------|-------|
| **Open Source + SaaS** | Lumibot (BotSpot.trade), QuantConnect | Most sustainable |
| **Open Source + Education** | Lumibot (AI Bootcamp), pysystemtrade | Good community building |
| **Open Source + Data Partnership** | Lumibot (ThetaData), CCXT (broker fees) | Passive revenue |
| **Open Source + Pro Version** | VectorBT Pro | Dual licensing |
| **Open Source + Enterprise** | NautilusTrader | High-value contracts |
| **Pure Open Source** | Basana, Barter-rs, HFTBacktest | Community goodwill |

---

## Part 8: Key Patterns & Ideas Worth Adopting

### From SimTradeLab
- Multi-level LRU caching for indicators
- AST analysis for smart data loading
- Pre-computed adjustment factors with HDF5

### From Meridian
- Determinism verification in CI
- Regime-adaptive slippage models
- Session-aware execution

### From microalpha
- First-class walk-forward support
- Bootstrap reality checks
- Factor regression integration

### From PyneCore
- Series/Persistent variable patterns
- AST transformation for DSL

### From QTradeX
- QPSO optimization engine
- Extreme performance focus

### From matching-core
- LMAX Disruptor pattern for performance
- SIMD optimization
- Zero-copy serialization

---

## Appendix: Complete Project Count by Category

| Category | Count | Notable Discovery |
|----------|-------|-------------------|
| Rust+Python Hybrid | 28 | Many new PyO3 projects emerging |
| Pure Rust | 50+ | matching-core (7M TPS) |
| Emerging Frameworks | 15+ | SimTradeLab (PTrade compat) |
| High-Perf Python | 30+ | GPU-accelerated Numba exists |
| Crypto-Specific | 2,600+ | Solana ecosystem exploding |
| Grafana Trading | 10 | All are monitoring, not backtest viz |
| DeFi Backtesting | 22 | demeter covers Uniswap/GMX/AAVE |
| Funding Rate Arb | 91 | Active area of development |
| Hyperliquid | 269 | Growing rapidly |

---

## Links by Category

### Rust+Python Hybrid (Must Watch)
- https://github.com/nautechsystems/nautilus_trader
- https://github.com/nkaz001/hftbacktest
- https://github.com/QuantML-C/pyrust-bt
- https://github.com/wingfoil-io/wingfoil
- https://github.com/jerryinyang/rustybt

### Pure Rust Trading
- https://github.com/barter-rs/barter-rs
- https://github.com/avhz/RustQuant
- https://github.com/greyblake/ta-rs
- https://github.com/amv-dev/yata
- https://github.com/llc-993/matching-core

### Emerging Frameworks
- https://github.com/kay-ou/SimTradeLab
- https://github.com/PyneSys/pynecore
- https://github.com/squidKid-deluxe/QTradeX-Algo-Trading-SDK
- https://github.com/brandonschabell/alphaflow
- https://github.com/MateoBodon/microalpha
- https://github.com/sukesan7/meridian

### DeFi & Crypto
- https://github.com/zelos-alpha/demeter
- https://github.com/aoki-h-jp/funding-rate-arbitrage
- https://github.com/warp-id/solana-trading-bot
- https://github.com/enarjord/passivbot

### Visualization
- https://github.com/thraizz/freqtrade-dashboard
- https://github.com/Finance-Insight-Lab/bt-visualizer
- https://github.com/ranaroussi/quantstats

---

*Research conducted: January 2026*
*Total unique projects discovered: ~2,700+*
