"""
Bollinger Bands Mean Reversion Strategy - VectorBT Reference Implementation

Strategy Logic:
- Buy when price touches or crosses below lower band (oversold)
- Sell when price touches or crosses above upper band (overbought)
- Exit at middle band (mean reversion)

Features Demonstrated:
- Bollinger Bands indicator
- Mean reversion trading
- Band width analysis
- %B indicator

VectorBT Concepts Used:
- vbt.BBANDS.run() - Bollinger Bands
- Band crossing detection
- Squeeze detection (low volatility)
"""

import vectorbt as vbt
import pandas as pd
import numpy as np


# ==============================================================================
# 1. BASIC BOLLINGER BANDS MEAN REVERSION
# ==============================================================================

def basic_bollinger_bands():
    """Simple BB mean reversion strategy"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate Bollinger Bands
    bb = vbt.BBANDS.run(
        price,
        window=20,  # Period
        alpha=2     # Standard deviations
    )

    # Entry: Price touches lower band (oversold)
    entries = price <= bb.lower

    # Exit: Price touches upper band or middle band
    exits = price >= bb.middle

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        init_cash=10_000,
        fees=0.001,
        freq='1D'
    )

    # Display results
    print("=" * 60)
    print("BOLLINGER BANDS MEAN REVERSION")
    print("=" * 60)
    print(f"Window: 20, Std Dev: 2")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")
    print(f"Avg Trade Duration: {pf.trades.duration.mean()}")

    # Plot
    pf.plot().show()

    return pf


# ==============================================================================
# 2. PARAMETER OPTIMIZATION (Window and Std Dev)
# ==============================================================================

def optimize_bollinger_parameters():
    """Test different BB parameters"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='2y').get('Close')

    # Test different parameters
    windows = [10, 15, 20, 25, 30]
    alphas = [1.5, 2.0, 2.5, 3.0]

    results = []

    for window in windows:
        for alpha in alphas:
            # Calculate Bollinger Bands
            bb = vbt.BBANDS.run(price, window=window, alpha=alpha)

            # Generate signals
            entries = price <= bb.lower
            exits = price >= bb.middle

            # Backtest
            pf = vbt.Portfolio.from_signals(
                price, entries, exits, fees=0.001
            )

            # Store results
            results.append({
                'window': window,
                'std_dev': alpha,
                'total_return': pf.total_return(),
                'sharpe': pf.sharpe_ratio(),
                'max_dd': pf.max_drawdown(),
                'win_rate': pf.trades.win_rate(),
                'num_trades': pf.trades.count()
            })

    # Convert to DataFrame
    results_df = pd.DataFrame(results)
    results_df = results_df.sort_values('sharpe', ascending=False)

    print("=" * 60)
    print("BOLLINGER BANDS PARAMETER OPTIMIZATION")
    print("=" * 60)
    print("\nTop 10 Configurations:")
    print(results_df.head(10).to_string(index=False))

    # Best configuration
    best = results_df.iloc[0]
    print(f"\nBest Configuration:")
    print(f"  Window: {best['window']}")
    print(f"  Std Dev: {best['std_dev']}")
    print(f"  Sharpe: {best['sharpe']:.2f}")
    print(f"  Total Return: {best['total_return']:.2%}")
    print(f"  Win Rate: {best['win_rate']:.2%}")

    # Visualize as heatmap
    pivot = results_df.pivot(index='window', columns='std_dev', values='sharpe')
    fig = pivot.vbt.heatmap(
        trace_kwargs=dict(colorscale='RdYlGn', zmid=0)
    )
    fig.show()

    return results_df


# ==============================================================================
# 3. BOLLINGER BANDS WITH RSI CONFIRMATION
# ==============================================================================

def bollinger_bands_rsi():
    """BB strategy with RSI confirmation"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate indicators
    bb = vbt.BBANDS.run(price, window=20, alpha=2)
    rsi = vbt.RSI.run(price, window=14)

    # Strategy 1: BB only
    entries_bb = price <= bb.lower
    exits_bb = price >= bb.middle

    pf_bb = vbt.Portfolio.from_signals(
        price, entries_bb, exits_bb, fees=0.001
    )

    # Strategy 2: BB + RSI confirmation
    entries_bb_rsi = (
        (price <= bb.lower) &  # Price at lower band
        (rsi.rsi < 30)         # RSI oversold
    )
    exits_bb_rsi = (
        (price >= bb.middle) |  # Price at middle band
        (rsi.rsi > 70)          # RSI overbought
    )

    pf_bb_rsi = vbt.Portfolio.from_signals(
        price, entries_bb_rsi, exits_bb_rsi, fees=0.001
    )

    print("=" * 60)
    print("BOLLINGER BANDS WITH/WITHOUT RSI")
    print("=" * 60)

    print("\nBB Only:")
    print(f"  Total Return: {pf_bb.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_bb.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_bb.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_bb.trades.win_rate():.2%}")
    print(f"  Trades: {pf_bb.trades.count()}")

    print("\nBB + RSI Confirmation:")
    print(f"  Total Return: {pf_bb_rsi.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_bb_rsi.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_bb_rsi.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_bb_rsi.trades.win_rate():.2%}")
    print(f"  Trades: {pf_bb_rsi.trades.count()}")

    return pf_bb, pf_bb_rsi


# ==============================================================================
# 4. BOLLINGER BANDS SQUEEZE (Low Volatility Breakout)
# ==============================================================================

def bollinger_squeeze_strategy():
    """Trade breakouts from BB squeeze (low volatility)"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate Bollinger Bands
    bb = vbt.BBANDS.run(price, window=20, alpha=2)

    # Calculate band width (volatility measure)
    bandwidth = (bb.upper - bb.lower) / bb.middle

    # Detect squeeze: bandwidth below 20th percentile
    squeeze_threshold = bandwidth.quantile(0.20)
    in_squeeze = bandwidth < squeeze_threshold

    # Calculate %B (where price is within bands)
    percent_b = (price - bb.lower) / (bb.upper - bb.lower)

    # Entry: Breakout from squeeze
    # Long when price breaks above middle band after squeeze
    squeeze_resolved = in_squeeze.shift(1) & ~in_squeeze
    entries = squeeze_resolved & (price > bb.middle)
    exits = price < bb.lower

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price, entries, exits, fees=0.001
    )

    print("=" * 60)
    print("BOLLINGER BANDS SQUEEZE STRATEGY")
    print("=" * 60)
    print(f"Squeeze Threshold: {squeeze_threshold:.4f}")
    print(f"Squeeze Periods: {in_squeeze.sum()}")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    return pf


# ==============================================================================
# 5. DOUBLE BOLLINGER BANDS STRATEGY
# ==============================================================================

def double_bollinger_bands():
    """Use two sets of BB with different std devs"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Inner bands (1 std dev)
    bb_inner = vbt.BBANDS.run(price, window=20, alpha=1)

    # Outer bands (2 std dev)
    bb_outer = vbt.BBANDS.run(price, window=20, alpha=2)

    # Strategy:
    # Strong buy: Price touches outer lower band
    # Weak buy: Price between inner and outer lower band
    # Exit: Price touches inner upper band

    strong_buy = price <= bb_outer.lower
    weak_buy = (price <= bb_inner.lower) & (price > bb_outer.lower)

    entries = strong_buy | weak_buy
    exits = price >= bb_inner.upper

    # Different position sizes
    # Strong buy: 100%, Weak buy: 50%
    size = np.where(strong_buy, 1.0, np.where(weak_buy, 0.5, 0))

    # Backtest (simplified - using from_signals)
    pf = vbt.Portfolio.from_signals(
        price, entries, exits, fees=0.001
    )

    print("=" * 60)
    print("DOUBLE BOLLINGER BANDS STRATEGY")
    print("=" * 60)
    print("Inner Bands: 1 std dev, Outer Bands: 2 std dev")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    return pf


# ==============================================================================
# 6. BOLLINGER BANDS %B STRATEGY
# ==============================================================================

def bollinger_percent_b_strategy():
    """Trade based on %B indicator"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='2y').get('Close')

    # Calculate Bollinger Bands
    bb = vbt.BBANDS.run(price, window=20, alpha=2)

    # Calculate %B: where price is within the bands
    # %B = (Price - Lower Band) / (Upper Band - Lower Band)
    # %B > 1: Above upper band
    # %B < 0: Below lower band
    # %B = 0.5: At middle band
    percent_b = (price - bb.lower) / (bb.upper - bb.lower)

    # Entry: %B crosses below 0 (price below lower band)
    entries = (percent_b < 0) & (percent_b.shift(1) >= 0)

    # Exit: %B crosses above 0.5 (price above middle band)
    exits = (percent_b > 0.5) & (percent_b.shift(1) <= 0.5)

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price, entries, exits, fees=0.001
    )

    print("=" * 60)
    print("BOLLINGER BANDS %B STRATEGY")
    print("=" * 60)
    print("Entry: %B crosses below 0")
    print("Exit: %B crosses above 0.5")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    # Analyze %B distribution at entries
    entry_percent_b = percent_b[entries]
    print(f"\n%B at Entry:")
    print(f"  Mean: {entry_percent_b.mean():.2f}")
    print(f"  Min: {entry_percent_b.min():.2f}")
    print(f"  Max: {entry_percent_b.max():.2f}")

    return pf


# ==============================================================================
# 7. BOLLINGER BANDS WITH VOLUME FILTER
# ==============================================================================

def bollinger_bands_volume():
    """BB strategy with volume confirmation"""

    # Download OHLCV data
    data = vbt.YFData.download('BTC-USD', period='2y')
    price = data.get('Close')
    volume = data.get('Volume')

    # Calculate indicators
    bb = vbt.BBANDS.run(price, window=20, alpha=2)

    # Volume MA for comparison
    volume_ma = vbt.MA.run(volume, window=20)

    # Entry: Price at lower band AND high volume
    entries = (
        (price <= bb.lower) &
        (volume > volume_ma.ma * 1.5)  # 50% above average volume
    )

    # Exit: Price at middle band
    exits = price >= bb.middle

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price, entries, exits, fees=0.001
    )

    print("=" * 60)
    print("BOLLINGER BANDS WITH VOLUME FILTER")
    print("=" * 60)
    print("Entry: Price at lower band + Volume > 1.5x average")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    return pf


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    print("\n" + "="*60)
    print("VECTORBT BOLLINGER BANDS EXAMPLES")
    print("="*60 + "\n")

    # 1. Basic mean reversion
    pf_basic = basic_bollinger_bands()

    # 2. Parameter optimization
    results = optimize_bollinger_parameters()

    # 3. With RSI confirmation
    pf_bb, pf_bb_rsi = bollinger_bands_rsi()

    # 4. Squeeze strategy
    pf_squeeze = bollinger_squeeze_strategy()

    # 5. Double Bollinger Bands
    pf_double = double_bollinger_bands()

    # 6. %B strategy
    pf_percent_b = bollinger_percent_b_strategy()

    # 7. Volume filter
    pf_volume = bollinger_bands_volume()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
