# VectorBT Strategy Reference Implementations

This directory contains Python reference implementations of popular trading strategies using VectorBT. These serve as the basis for migration to ZenBT (Rust).

## Purpose

These files demonstrate **how strategies are implemented in VectorBT** so they can be accurately translated to ZenBT's Rust-based architecture. Each file is fully functional and can be run independently.

## Files

### 01_ma_crossover.py
**Moving Average Crossover Strategies**

- Basic MA crossover (10/50)
- Parameter optimization (testing 100+ combinations)
- Multi-asset application
- Stop loss and take profit integration
- Walk-forward analysis

**Key VectorBT Features:**
- `vbt.MA.run()`
- `ma_crossed_above()` / `ma_crossed_below()`
- `run_combs()` for parameter testing
- `Portfolio.from_signals()`

**Migration Complexity:** Low
**Rust Implementation:** Similar to `src/zenbt/strategies/ma_cross.py`

---

### 02_rsi_macd.py
**RSI + MACD Combination Strategies**

- Basic RSI + MACD combination
- RSI threshold optimization
- MACD parameter optimization
- Trend filter (200 MA)
- Divergence detection
- Multi-timeframe confirmation

**Key VectorBT Features:**
- `vbt.RSI.run()`
- `vbt.MACD.run()`
- Boolean operators (`&`, `|`) for signal combination
- Multi-timeframe resampling

**Migration Complexity:** Low-Medium
**Challenges:** Divergence detection requires state tracking

---

### 03_bollinger_bands.py
**Bollinger Bands Mean Reversion**

- Basic BB mean reversion
- Parameter optimization (window, std dev)
- RSI confirmation
- Squeeze detection (low volatility breakout)
- Double Bollinger Bands
- %B indicator strategy
- Volume filter

**Key VectorBT Features:**
- `vbt.BBANDS.run()`
- Band width calculations
- %B indicator
- Volume analysis

**Migration Complexity:** Low-Medium
**Challenges:** %B calculation, band width tracking

---

### 04_trailing_stop.py
**Stop Loss and Trailing Stop Strategies**

- Fixed vs trailing stop comparison
- Stop distance optimization
- Stop loss + take profit combinations
- ATR-based dynamic stops
- Chandelier exit
- Multi-level profit targets

**Key VectorBT Features:**
- `sl_stop` parameter
- `tp_stop` parameter
- `sl_trail=True` for trailing
- ATR-based calculations

**Migration Complexity:** Medium
**Challenges:** ZenBT has built-in SL/TP but may need custom trailing logic

---

### 05_custom_order_func.py
**Advanced Custom Order Logic**

- Basic custom order function
- Volatility-based position sizing
- Pyramiding (adding to winners)
- Grid trading
- Martingale strategy

**Key VectorBT Features:**
- `Portfolio.from_order_func()`
- Numba `@njit` decorators
- Context object (`c`) for state access
- Dynamic position sizing

**Migration Complexity:** High
**Challenges:** Requires full rewrite using `on_candle()` with state management

---

### 06_multi_asset.py
**Multi-Asset Portfolio Strategies**

- Basic multi-asset strategy
- Cash sharing portfolios
- Equal weight portfolios
- Correlation-based selection
- Sector rotation
- Mean-variance optimization
- Dynamic position sizing across assets

**Key VectorBT Features:**
- Multi-column DataFrames
- `group_by=True` for portfolio aggregation
- `cash_sharing=True` parameter
- PyPortfolioOpt integration
- Correlation analysis

**Migration Complexity:** Medium-High
**Challenges:** Portfolio-level state management, rebalancing logic

---

### 07_pairs_trading.py
**Pairs Trading and Statistical Arbitrage**

- Find cointegrated pairs
- Basic pairs trading
- Hedge ratio optimization
- Rolling hedge ratio
- Multiple pairs portfolio
- Kalman filter pairs trading
- Distance method

**Key VectorBT Features:**
- Cointegration testing (statsmodels)
- Spread calculation
- Z-score trading
- Simultaneous long/short positions
- Kalman Filter integration

**Migration Complexity:** High
**Challenges:** Simultaneous long/short execution, hedge ratio calculation

---

### 08_walk_forward.py
**Walk-Forward Optimization & Cross-Validation**

- Basic walk-forward
- Anchored walk-forward (expanding window)
- Combinatorial cross-validation
- Out-of-sample decay analysis
- Monte Carlo walk-forward
- Parameter stability testing

**Key VectorBT Features:**
- Time-based splits
- Rolling/expanding windows
- Multiple validation methods
- Overfitting detection
- Parameter stability analysis

**Migration Complexity:** Medium
**Challenges:** Requires multi_backtest framework, split management

---

## Running the Examples

### Prerequisites

```bash
pip install vectorbt yfinance ta-lib pandas numpy
```

### Run Individual Files

```bash
python _tdd/docs/01_ma_crossover.py
python _tdd/docs/02_rsi_macd.py
python _tdd/docs/03_bollinger_bands.py
python _tdd/docs/04_trailing_stop.py
python _tdd/docs/05_custom_order_func.py
```

Each file is standalone and will:
1. Download required data from Yahoo Finance
2. Run multiple strategy variations
3. Display performance metrics
4. Generate plots (if running in interactive mode)

## Migration Guide

### Step 1: Understand VectorBT Implementation

Read the Python file to understand:
- What indicators are used
- How signals are generated
- What the entry/exit logic is
- Any special features (stops, sizing, etc.)

### Step 2: Identify Required Components

For ZenBT, you'll need:
- **Indicators**: Which TA-Lib or custom indicators?
- **Data columns**: What needs to be added to DataFrame?
- **State**: What needs to be tracked between candles?
- **Orders**: Market, limit, or stop orders?

### Step 3: Translate to ZenBT Pattern

VectorBT pattern:
```python
# Calculate all indicators
rsi = vbt.RSI.run(price, 14)
macd = vbt.MACD.run(price, 12, 26, 9)

# Generate signals (boolean arrays)
entries = (rsi.rsi_below(30) & macd.macd_above(macd.signal))
exits = (rsi.rsi_above(70) | macd.macd_below(macd.signal))

# Backtest
pf = vbt.Portfolio.from_signals(price, entries, exits)
```

ZenBT equivalent:
```python
# 1. Calculate indicators upfront (add to DataFrame)
df = df.with_columns([
    pl.Series("rsi", talib.RSI(df["close"], 14)),
    pl.Series("macd", talib.MACD(df["close"], 12, 26, 9)[0]),
    pl.Series("macd_signal", talib.MACD(df["close"], 12, 26, 9)[1]),
])

# 2. Define strategy class
class RSI_MACD(BaseStrategy):
    default_size = 1000

    def on_candle(self, state=None):
        # Check current candle conditions
        rsi_oversold = self.data["rsi"][self.index] < 30
        macd_bullish = self.data["macd"][self.index] > self.data["macd_signal"][self.index]

        # Entry logic
        if rsi_oversold and macd_bullish:
            order = self.create_market_order(
                self.index,
                client_order_id="long",
                side=Side.Long,
                size=self.default_size
            )
            return Action(orders={"long": order})

        # Exit logic
        rsi_overbought = self.data["rsi"][self.index] > 70
        macd_bearish = self.data["macd"][self.index] < self.data["macd_signal"][self.index]

        if rsi_overbought or macd_bearish:
            return Action(close_all_positions=True)

        return self.action

# 3. Run backtest
strategy = RSI_MACD(df, default_size=1000)
params = BacktestParams(commission_pct=0.001, initial_capital=10000)
bt = Backtest(df, params, strategy)
bt.backtest()

# 4. Get stats
stats = Stats(bt)
stats.print()
```

### Step 4: Handle Special Cases

#### Crossover Detection

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
df = df.with_columns([
    pl.Series("cross_above", zbt.indicators.cross_above(fast_ma, slow_ma))
])
```

#### Stop Loss / Take Profit

**VectorBT:**
```python
pf = vbt.Portfolio.from_signals(price, entries, exits,
    sl_stop=0.05,
    tp_stop=0.10)
```

**ZenBT:**
```python
entry_price = self.data["close"][self.index]
order = self.create_market_order(
    self.index,
    client_order_id="long",
    side=Side.Long,
    size=1000,
    sl=entry_price * 0.95,  # 5% stop loss
    tp=entry_price * 1.10   # 10% take profit
)
```

#### State Tracking

**VectorBT:** Implicit (managed by Portfolio)

**ZenBT:** Explicit via `state` parameter
```python
def on_candle(self, state=None):
    # Access current equity
    current_equity = state.equity

    # Access active positions
    for pos_id, position in state.active_positions.items():
        # Check position PnL, entry price, etc.
        pass
```

### Step 5: Test and Validate

1. Run VectorBT version, note results
2. Run ZenBT version
3. Compare:
   - Total P&L (should be within 1-2%)
   - Number of trades (may differ slightly)
   - Win rate
   - Max drawdown

Expect minor differences due to execution model (VectorBT: close-to-close, ZenBT: next-open)

## Common Patterns Quick Reference

| VectorBT | ZenBT |
|----------|-------|
| `vbt.MA.run(price, 20)` | `talib.SMA(df["close"], 20)` |
| `vbt.RSI.run(price, 14)` | `talib.RSI(df["close"], 14)` |
| `vbt.MACD.run(price, 12, 26, 9)` | `talib.MACD(df["close"], 12, 26, 9)` |
| `vbt.BBANDS.run(price, 20, 2)` | `talib.BBANDS(df["close"], 20, 2, 2)` |
| `ma.ma_crossed_above(ma2)` | Check current > and prev <= |
| `rsi.rsi_below(30)` | `self.data["rsi"][self.index] < 30` |
| `pf.from_signals()` | `Backtest(df, params, strategy)` |
| `pf.total_return()` | `stats.stats.pnl_pct` |
| `pf.sharpe_ratio()` | Not built-in (calculate manually) |

## Tips for Migration

1. **Start Simple**: Begin with basic strategies (MA crossover) before complex ones
2. **Test Indicators**: Verify TA-Lib outputs match VectorBT
3. **Debug Per-Candle**: Print values at specific indices to verify logic
4. **Use Type Hints**: Rust is strongly typed; Python type hints help
5. **Profile Performance**: ZenBT should be faster; if not, investigate

## Next Steps

After reviewing these files:

1. Choose a strategy to implement
2. Create new file in `src/zenbt/strategies/`
3. Follow migration pattern from `STRATEGY_MIGRATION.md`
4. Test with sample data
5. Compare results with VectorBT
6. Document any differences or challenges

## Resources

- **VectorBT Docs**: https://vectorbt.dev/
- **TA-Lib**: https://ta-lib.github.io/ta-lib-python/
- **ZenBT Example**: `src/zenbt/strategies/ma_cross.py`
- **Migration Guide**: `_tdd/STRATEGY_MIGRATION.md`
