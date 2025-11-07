# VectorBT to ZenBT Strategy Migration Guide

## Overview

This document provides a comprehensive guide for migrating trading strategies from VectorBT (Python) to ZenBT (Rust). The goal is to replicate popular VectorBT strategies using ZenBT's high-performance Rust engine while maintaining equivalent functionality.

## Architecture Comparison

### VectorBT Architecture
```
Python Strategy → NumPy Arrays → Vectorized Operations → Pandas DataFrames
```

### ZenBT Architecture
```
Rust Strategy → Polars DataFrames → Rust Engine → Position Management
```

## Key Differences

| Feature | VectorBT | ZenBT |
|---------|----------|-------|
| **Language** | Python | Rust + Python bindings |
| **Data Structure** | Pandas/NumPy | Polars/NumPy (via PyO3) |
| **Execution** | Vectorized | Candle-by-candle iteration |
| **Signal Generation** | Array-based (all at once) | Event-driven (per candle) |
| **Order Execution** | Portfolio.from_signals() | Action-based order creation |
| **Speed** | Fast (vectorized) | Faster (compiled Rust) |

## Migration Strategy

### 1. Signal Generation Pattern

**VectorBT:**
```python
# Generate all signals upfront
fast_ma = vbt.MA.run(price, 10)
slow_ma = vbt.MA.run(price, 50)

entries = fast_ma.ma_crossed_above(slow_ma)  # Boolean array
exits = fast_ma.ma_crossed_below(slow_ma)    # Boolean array

pf = vbt.Portfolio.from_signals(price, entries, exits)
```

**ZenBT:**
```python
# Calculate indicators upfront, check per candle
fast_ma = talib.SMA(df["close"], 10)
slow_ma = talib.SMA(df["close"], 50)

class MACross(BaseStrategy):
    def on_candle(self, state=None):
        # Check current candle
        cross_above = (
            self.data["fast_ma"][self.index] > self.data["slow_ma"][self.index] and
            self.data["fast_ma"][self.index - 1] <= self.data["slow_ma"][self.index - 1]
        )

        if cross_above:
            order = self.create_market_order(...)
            return Action(orders={...})

        return self.action
```

### 2. Data Preparation

**VectorBT:**
```python
# Download and use directly
price = vbt.YFData.download('BTC-USD').get('Close')
```

**ZenBT:**
```python
# Load from Parquet and add indicators
df = read_data_pl("BTC", exchange="binance")
df = df.with_columns([
    pl.Series("fast_ma", talib.SMA(df["close"], 10)),
    pl.Series("slow_ma", talib.SMA(df["close"], 50)),
])
```

### 3. Strategy Structure

**VectorBT Pattern:**
- Calculate all indicators
- Generate all signals (boolean arrays)
- Pass to Portfolio.from_signals()
- Analyze results

**ZenBT Pattern:**
- Load data
- Calculate all indicators (add as DataFrame columns)
- Define strategy class with on_candle() method
- Create Backtest instance
- Run backtest
- Analyze stats

### 4. Order Management

**VectorBT:**
```python
# Implicit order management
pf = vbt.Portfolio.from_signals(
    price,
    entries=buy_signals,
    exits=sell_signals,
    sl_stop=0.05,  # 5% stop loss
    tp_stop=0.10   # 10% take profit
)
```

**ZenBT:**
```python
# Explicit order creation
def on_candle(self, state=None):
    if buy_signal:
        entry_price = self.data["close"][self.index]
        order = self.create_market_order(
            self.index,
            client_order_id="long",
            side=Side.Long,
            size=self.default_size,
            sl=entry_price * 0.95,  # 5% stop loss
            tp=entry_price * 1.10   # 10% take profit
        )
        return Action(
            orders={"long": order},
            close_all_positions=True
        )
```

### 5. Performance Metrics

**VectorBT:**
```python
pf.total_return()
pf.sharpe_ratio()
pf.max_drawdown()
pf.stats()
```

**ZenBT:**
```python
stats = Stats(bt)
stats.print()  # Shows all metrics
# Access via stats.stats object:
# - pnl, pnl_pct
# - win_rate
# - max_drawdown, max_drawdown_pct
# - total_positions, wins, losses
```

## Strategy Implementations

The `_tdd/docs/` directory contains Python implementations of popular VectorBT strategies. Each file follows this structure:

```
01_ma_crossover.py          - Simple moving average crossover
02_rsi_macd.py              - RSI + MACD combination
03_bollinger_bands.py       - Bollinger Bands mean reversion
04_trailing_stop.py         - Trailing stop implementation
05_multi_asset.py           - Multi-asset portfolio
06_walk_forward.py          - Walk-forward optimization
07_custom_order_func.py     - Advanced custom logic
08_pairs_trading.py         - Pairs trading strategy
```

## Migration Checklist

For each strategy migration:

- [ ] Identify all indicators used
- [ ] Calculate indicators upfront (add as DataFrame columns)
- [ ] Convert array-based signals to per-candle logic
- [ ] Implement on_candle() method with signal detection
- [ ] Handle order creation (market/limit/stop)
- [ ] Implement stop loss / take profit logic
- [ ] Test with sample data
- [ ] Compare results with VectorBT version
- [ ] Optimize performance if needed
- [ ] Document any behavioral differences

## Common Patterns

### Pattern 1: Crossover Detection

**VectorBT:**
```python
entries = fast_ma.ma_crossed_above(slow_ma)
```

**ZenBT:**
```python
cross_above = (
    self.data["fast_ma"][self.index] > self.data["slow_ma"][self.index] and
    self.data["fast_ma"][self.index - 1] <= self.data["slow_ma"][self.index - 1]
)
```

Or use built-in:
```python
cross_above = zbt.indicators.cross_above(fast_ma_series, slow_ma_series)
```

### Pattern 2: Threshold Checks

**VectorBT:**
```python
entries = rsi.rsi_below(30)
exits = rsi.rsi_above(70)
```

**ZenBT:**
```python
oversold = self.data["rsi"][self.index] < 30
overbought = self.data["rsi"][self.index] > 70
```

### Pattern 3: Multiple Conditions

**VectorBT:**
```python
entries = (
    rsi.rsi_below(30) &
    macd.macd_above(macd.signal) &
    volume > volume_ma
)
```

**ZenBT:**
```python
buy_signal = (
    self.data["rsi"][self.index] < 30 and
    self.data["macd"][self.index] > self.data["macd_signal"][self.index] and
    self.data["volume"][self.index] > self.data["volume_ma"][self.index]
)
```

## Optimization Approach

### VectorBT: Grid Search (Vectorized)
```python
# Test all combinations at once
windows = np.arange(2, 101)
fast_ma, slow_ma = vbt.MA.run_combs(price, window=windows, r=2)
pf = vbt.Portfolio.from_signals(price, entries, exits)
best_params = pf.sharpe_ratio().idxmax()
```

### ZenBT: Parallel Backtesting
```python
# Use multi_backtest for parallel execution
from zenbt.multi_backtest import multi_backtest

params = [
    (10, 20), (10, 30), (10, 50),  # (fast, slow) windows
    (20, 50), (20, 100), (50, 100)
]

def run_ma_strategy(params, df, ohlcvs, args):
    fast_window, slow_window = params
    # Calculate indicators with these params
    # Run backtest
    # Return results

results = multi_backtest(df, ohlcvs, params, run_ma_strategy)
```

## Performance Expectations

| Strategy Complexity | VectorBT Speed | ZenBT Speed | Migration Effort |
|---------------------|----------------|-------------|------------------|
| Simple MA Cross     | Very Fast      | Fast        | Low              |
| RSI + MACD          | Very Fast      | Fast        | Low              |
| Bollinger Bands     | Very Fast      | Fast        | Low              |
| Custom Order Logic  | Fast           | Very Fast   | Medium           |
| Multi-Asset         | Fast           | Fast        | Medium           |
| Portfolio Rebal.    | Medium         | Fast        | High             |

## Limitations & Workarounds

### VectorBT Feature: Portfolio Rebalancing
**Status:** Not directly supported in ZenBT
**Workaround:** Implement custom position sizing logic in on_candle()

### VectorBT Feature: Vectorized Operations
**Status:** Not applicable (different architecture)
**Alternative:** Indicators calculated upfront with NumPy/TA-Lib

### VectorBT Feature: Interactive Plotly Dashboards
**Status:** Not built-in
**Alternative:** Use mplfinance, Bokeh, or ECharts/Grafana

### VectorBT Feature: Walk-Forward CV
**Status:** Manual implementation required
**Alternative:** Use multi_backtest with custom split logic

## Testing Strategy

Each migrated strategy should be tested:

1. **Unit Tests**: Test signal generation logic
2. **Integration Tests**: Compare P&L with VectorBT (allow for small differences due to execution model)
3. **Performance Tests**: Verify Rust performance advantage
4. **Edge Cases**: Test with gaps, low volume, extreme prices

## Expected Results Differences

Due to architectural differences, expect slight variations:

- **Execution timing**: VectorBT uses close-to-close, ZenBT uses next-candle-open
- **Simultaneous signals**: VectorBT handles arrays, ZenBT processes sequentially
- **Slippage model**: Different implementations
- **Floating point precision**: Rust uses Decimal, Python uses float64

Typically, results should match within 1-2% for total returns.

## Next Steps

1. Review Python strategy files in `_tdd/docs/`
2. For each strategy:
   - Understand the logic
   - Identify required indicators
   - Implement in Rust/ZenBT
   - Test and validate
3. Create Rust implementations in `src/zenbt/strategies/`
4. Document any findings or issues

## Resources

- **ZenBT Examples**: `/home/user/zenbt/src/zenbt/strategies/ma_cross.py`
- **VectorBT Docs**: https://vectorbt.dev/
- **Polars API**: https://docs.pola.rs/
- **TA-Lib**: https://ta-lib.github.io/ta-lib-python/

## Contributing

When adding new strategy migrations:

1. Add Python reference implementation to `_tdd/docs/`
2. Document any VectorBT-specific features used
3. Implement Rust version in `src/zenbt/strategies/`
4. Add comparison tests
5. Update this document with lessons learned
