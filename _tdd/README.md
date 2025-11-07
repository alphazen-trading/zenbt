# Technical Design Document - VectorBT to ZenBT Strategy Migration

## Overview

This directory contains technical documentation and reference implementations for migrating trading strategies from VectorBT (Python) to ZenBT (Rust).

**Objective**: Provide comprehensive examples of VectorBT strategies in Python that can serve as templates for Rust implementations in ZenBT.

## Directory Structure

```
_tdd/
├── README.md                    # This file
├── STRATEGY_MIGRATION.md        # Detailed migration guide
└── docs/                        # Python strategy references
    ├── README.md                # Strategy documentation
    ├── 01_ma_crossover.py       # MA crossover examples
    ├── 02_rsi_macd.py          # RSI + MACD combinations
    ├── 03_bollinger_bands.py   # Bollinger Bands strategies
    ├── 04_trailing_stop.py     # Stop loss variations
    └── 05_custom_order_func.py # Advanced custom logic
```

## Quick Start

### 1. Review the Migration Guide

Start with `STRATEGY_MIGRATION.md` to understand:
- Architecture differences between VectorBT and ZenBT
- Signal generation patterns
- Data preparation approaches
- Common migration patterns

### 2. Explore Strategy Examples

Each file in `docs/` contains multiple strategy variations:

| File | Strategies | Complexity | Lines |
|------|-----------|------------|-------|
| `01_ma_crossover.py` | 5 variations | Low | ~300 |
| `02_rsi_macd.py` | 6 variations | Low-Med | ~400 |
| `03_bollinger_bands.py` | 7 variations | Low-Med | ~450 |
| `04_trailing_stop.py` | 6 variations | Medium | ~400 |
| `05_custom_order_func.py` | 5 variations | High | ~500 |

**Total**: 29 strategy variations across 5 files

### 3. Run Examples

```bash
# Install dependencies
pip install vectorbt yfinance ta-lib pandas numpy

# Run any example
cd /home/user/zenbt
python _tdd/docs/01_ma_crossover.py
```

Each script will:
- Download BTC/ETH data from Yahoo Finance
- Run multiple strategy variations
- Display performance metrics
- Show comparison between variations

### 4. Begin Migration

1. Choose a strategy from `docs/`
2. Understand the VectorBT implementation
3. Create equivalent in `src/zenbt/strategies/`
4. Test and validate results

## Key Documents

### STRATEGY_MIGRATION.md

**Comprehensive migration guide covering:**

- Architecture comparison (VectorBT vs ZenBT)
- Key differences in execution models
- Signal generation patterns
- Data preparation strategies
- Order management approaches
- Performance metrics mapping
- Common migration patterns
- Optimization approaches
- Testing strategies

**Sections:**
1. Overview
2. Architecture Comparison
3. Key Differences
4. Migration Strategy (signal patterns, data prep, structure)
5. Strategy Implementations (file descriptions)
6. Migration Checklist
7. Common Patterns
8. Optimization Approach
9. Performance Expectations
10. Limitations & Workarounds
11. Testing Strategy
12. Expected Results Differences
13. Next Steps
14. Resources

### docs/README.md

**Strategy reference guide:**

- Detailed description of each strategy file
- VectorBT features used
- Migration complexity ratings
- Running instructions
- Migration step-by-step guide
- Common patterns quick reference
- Tips and best practices

## Strategy Coverage

### Simple Strategies (Easy Migration)

**Moving Average Crossover** (`01_ma_crossover.py`)
- ✅ Basic 2-MA crossover
- ✅ Parameter optimization
- ✅ Multi-asset
- ✅ With stops
- ✅ Walk-forward analysis

**RSI + MACD** (`02_rsi_macd.py`)
- ✅ Basic combination
- ✅ Threshold optimization
- ✅ Parameter optimization
- ✅ Trend filter
- ✅ Divergence detection
- ✅ Multi-timeframe

**Bollinger Bands** (`03_bollinger_bands.py`)
- ✅ Mean reversion
- ✅ Parameter optimization
- ✅ RSI confirmation
- ✅ Squeeze detection
- ✅ Double BB
- ✅ %B indicator
- ✅ Volume filter

### Intermediate Strategies

**Trailing Stops** (`04_trailing_stop.py`)
- ✅ Fixed vs trailing comparison
- ✅ Distance optimization
- ✅ SL/TP combinations
- ✅ ATR-based stops
- ✅ Chandelier exit
- ✅ Multi-level targets

### Advanced Strategies (Complex Migration)

**Custom Order Functions** (`05_custom_order_func.py`)
- ✅ Dynamic position sizing
- ✅ Volatility-based sizing
- ✅ Pyramiding
- ✅ Grid trading
- ✅ Martingale (educational)

## Migration Complexity Matrix

| Strategy Type | VectorBT Complexity | ZenBT Complexity | Migration Effort |
|---------------|---------------------|------------------|------------------|
| MA Crossover | Simple | Simple | Low (1-2 hours) |
| RSI/MACD | Simple | Simple | Low (2-3 hours) |
| Bollinger Bands | Simple | Medium | Medium (3-4 hours) |
| Trailing Stops | Medium | Medium | Medium (4-6 hours) |
| Custom Order Func | High | High | High (8-12 hours) |

## VectorBT Features Used

### Indicators
- ✅ `vbt.MA.run()` - Moving averages
- ✅ `vbt.RSI.run()` - RSI
- ✅ `vbt.MACD.run()` - MACD
- ✅ `vbt.BBANDS.run()` - Bollinger Bands
- ✅ `vbt.ATR.run()` - Average True Range

### Backtesting Methods
- ✅ `Portfolio.from_signals()` - Signal-based (most common)
- ✅ `Portfolio.from_order_func()` - Custom logic
- ✅ `run_combs()` - Parameter combinations

### Signal Detection
- ✅ `ma_crossed_above()` / `ma_crossed_below()`
- ✅ `rsi_below()` / `rsi_above()`
- ✅ `macd_above()` / `macd_below()`
- ✅ Boolean operators (`&`, `|`, `~`)

### Risk Management
- ✅ `sl_stop` - Stop loss
- ✅ `tp_stop` - Take profit
- ✅ `sl_trail` - Trailing stop
- ✅ Dynamic position sizing

### Analysis
- ✅ `total_return()`
- ✅ `sharpe_ratio()`
- ✅ `max_drawdown()`
- ✅ `trades.win_rate()`
- ✅ `trades.count()`

## ZenBT Equivalents

| VectorBT | ZenBT Equivalent |
|----------|------------------|
| `vbt.MA.run(price, 20)` | `talib.SMA(df["close"], 20)` |
| `vbt.RSI.run(price, 14)` | `talib.RSI(df["close"], 14)` |
| `vbt.MACD.run(...)` | `talib.MACD(df["close"], 12, 26, 9)` |
| `Portfolio.from_signals()` | `Backtest(df, params, strategy).backtest()` |
| `ma_crossed_above()` | Custom check or `zbt.indicators.cross_above()` |
| `pf.total_return()` | `stats.stats.pnl_pct` |
| `pf.sharpe_ratio()` | Manual calculation |
| Signal arrays | Per-candle logic in `on_candle()` |

## Implementation Pattern

### VectorBT (Array-Based)
```python
# 1. Calculate indicators (entire series)
fast_ma = vbt.MA.run(price, 10)
slow_ma = vbt.MA.run(price, 50)

# 2. Generate signals (boolean arrays)
entries = fast_ma.ma_crossed_above(slow_ma)
exits = fast_ma.ma_crossed_below(slow_ma)

# 3. Backtest
pf = vbt.Portfolio.from_signals(price, entries, exits)

# 4. Analyze
print(pf.total_return())
```

### ZenBT (Event-Driven)
```python
# 1. Prepare data with indicators
df = df.with_columns([
    pl.Series("fast_ma", talib.SMA(df["close"], 10)),
    pl.Series("slow_ma", talib.SMA(df["close"], 50)),
])

# 2. Define strategy class
class MACross(BaseStrategy):
    def on_candle(self, state=None):
        # Check crossover at current candle
        if (self.data["fast_ma"][self.index] > self.data["slow_ma"][self.index] and
            self.data["fast_ma"][self.index-1] <= self.data["slow_ma"][self.index-1]):
            order = self.create_market_order(...)
            return Action(orders={...})
        return self.action

# 3. Run backtest
bt = Backtest(df, params, MACross(df))
bt.backtest()

# 4. Analyze
Stats(bt).print()
```

## Testing Approach

### Validation Checklist

For each migrated strategy:

- [ ] Indicators match (compare values at random indices)
- [ ] Signals match (compare entry/exit points)
- [ ] Trade count similar (±5%)
- [ ] Total return similar (±2%)
- [ ] Win rate similar (±3%)
- [ ] Max drawdown similar (±2%)
- [ ] Performance faster in Rust

### Debugging Tips

1. **Compare Indicator Values**: Print first 10 values from both
2. **Check Signal Timing**: Print indices where trades occur
3. **Verify Order Execution**: Compare entry/exit prices
4. **Profile Performance**: Time both implementations

## Performance Benchmarks

Expected speedups (ZenBT vs VectorBT):

| Operation | VectorBT | ZenBT | Speedup |
|-----------|----------|-------|---------|
| Backtest (100K bars) | ~2s | ~0.2s | 10x |
| Indicator calculation | ~0.1s | ~0.05s | 2x |
| Parameter optimization | ~30s | ~3s | 10x |
| Memory usage | ~500MB | ~100MB | 5x |

## Next Steps

### For Strategy Migration

1. **Choose Strategy**: Start with `01_ma_crossover.py`
2. **Run VectorBT Version**: Understand baseline
3. **Create ZenBT Version**: In `src/zenbt/strategies/`
4. **Test**: Compare results
5. **Optimize**: If needed
6. **Document**: Note any differences

### For Adding New Strategies

1. **Research**: Find VectorBT implementation
2. **Add to docs/**: Create `06_new_strategy.py`
3. **Document**: Update `docs/README.md`
4. **Migrate**: Implement in ZenBT
5. **Test**: Validate results
6. **Share**: Update this README

## Resources

- **VectorBT Documentation**: https://vectorbt.dev/
- **VectorBT Pro**: https://vectorbt.pro/
- **TA-Lib Python**: https://ta-lib.github.io/ta-lib-python/
- **Polars**: https://docs.pola.rs/
- **ZenBT MA Example**: `src/zenbt/strategies/ma_cross.py`

## Contributing

When adding new strategy examples:

1. Follow existing file structure
2. Include multiple variations
3. Add comprehensive comments
4. Document VectorBT features used
5. Rate migration complexity
6. Update this README

## Notes

- All VectorBT examples download data automatically (Yahoo Finance)
- Strategies are educational; not investment advice
- Performance may vary with different parameters
- Backtest results don't guarantee future performance

## Summary Statistics

- **Total Strategy Files**: 5
- **Total Strategy Variations**: 29
- **Total Lines of Code**: ~2,100
- **Coverage**: Entry-level to advanced
- **Estimated Migration Time**: 30-50 hours for all

---

Created: 2025-11-07
Last Updated: 2025-11-07
Version: 1.0.0
