"""
RSI + MACD Combination Strategy - VectorBT Reference Implementation

Strategy Logic:
- Entry: RSI oversold (<30) AND MACD bullish (above signal line)
- Exit: RSI overbought (>70) OR MACD bearish (below signal line)

Features Demonstrated:
- Multiple indicator combination
- Boolean logic for signal generation
- Divergence detection (optional)

VectorBT Concepts Used:
- vbt.RSI.run() - RSI indicator
- vbt.MACD.run() - MACD indicator
- Boolean operators (&, |, ~) for signal combination
- Parameter optimization with multiple indicators
"""

import vectorbt as vbt
import pandas as pd
import numpy as np
from itertools import product, combinations


# ==============================================================================
# 1. BASIC RSI + MACD STRATEGY
# ==============================================================================

def basic_rsi_macd():
    """Simple RSI + MACD combination strategy"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate indicators
    rsi = vbt.RSI.run(price, window=14)
    macd = vbt.MACD.run(price, fast_window=12, slow_window=26, signal_window=9)

    # Entry: RSI oversold AND MACD bullish
    entries = (
        rsi.rsi_below(30) &  # RSI < 30 (oversold)
        macd.macd_above(macd.signal)  # MACD > signal (bullish)
    )

    # Exit: RSI overbought OR MACD bearish
    exits = (
        rsi.rsi_above(70) |  # RSI > 70 (overbought)
        macd.macd_below(macd.signal)  # MACD < signal (bearish)
    )

    # Run backtest
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
    print("RSI + MACD COMBINATION STRATEGY")
    print("=" * 60)
    print(f"RSI Period: 14, Oversold: 30, Overbought: 70")
    print(f"MACD: 12/26/9")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")
    print(f"Avg Trade Duration: {pf.trades.duration.mean()}")

    # Analyze trade distribution
    print(f"\nTrade Analysis:")
    print(f"  Avg Win: {pf.trades.winning.avg_return():.2%}")
    print(f"  Avg Loss: {pf.trades.losing.avg_return():.2%}")
    print(f"  Profit Factor: {pf.trades.profit_factor():.2f}")

    # Plot
    pf.plot().show()

    return pf


# ==============================================================================
# 2. OPTIMIZE RSI THRESHOLDS
# ==============================================================================

def optimize_rsi_thresholds():
    """Test different RSI overbought/oversold levels"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Test different RSI thresholds
    oversold_levels = np.arange(20, 41, 5)  # 20, 25, 30, 35, 40
    overbought_levels = np.arange(60, 81, 5)  # 60, 65, 70, 75, 80

    # Calculate MACD once
    macd = vbt.MACD.run(price, fast_window=12, slow_window=26, signal_window=9)

    results = []

    for oversold, overbought in product(oversold_levels, overbought_levels):
        if oversold >= overbought:
            continue

        # Calculate RSI
        rsi = vbt.RSI.run(price, window=14)

        # Generate signals
        entries = rsi.rsi_below(oversold) & macd.macd_above(macd.signal)
        exits = rsi.rsi_above(overbought) | macd.macd_below(macd.signal)

        # Backtest
        pf = vbt.Portfolio.from_signals(
            price, entries, exits, fees=0.001
        )

        # Store results
        results.append({
            'oversold': oversold,
            'overbought': overbought,
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
    print("RSI THRESHOLD OPTIMIZATION")
    print("=" * 60)
    print("\nTop 5 Configurations:")
    print(results_df.head(10).to_string(index=False))

    # Best configuration
    best = results_df.iloc[0]
    print(f"\nBest Configuration:")
    print(f"  Oversold: {best['oversold']}")
    print(f"  Overbought: {best['overbought']}")
    print(f"  Sharpe: {best['sharpe']:.2f}")
    print(f"  Total Return: {best['total_return']:.2%}")
    print(f"  Win Rate: {best['win_rate']:.2%}")

    return results_df


# ==============================================================================
# 3. OPTIMIZE MACD PARAMETERS
# ==============================================================================

def optimize_macd_parameters():
    """Test different MACD window combinations"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='2y').get('Close')

    # Calculate RSI once
    rsi = vbt.RSI.run(price, window=14)

    # Test MACD parameters
    fast_windows = [8, 12, 16]
    slow_windows = [20, 26, 32]
    signal_windows = [7, 9, 11]

    results = []

    for fast, slow, signal in product(fast_windows, slow_windows, signal_windows):
        if fast >= slow:
            continue

        # Calculate MACD
        macd = vbt.MACD.run(
            price,
            fast_window=fast,
            slow_window=slow,
            signal_window=signal
        )

        # Generate signals
        entries = rsi.rsi_below(30) & macd.macd_above(macd.signal)
        exits = rsi.rsi_above(70) | macd.macd_below(macd.signal)

        # Backtest
        pf = vbt.Portfolio.from_signals(
            price, entries, exits, fees=0.001
        )

        # Store results
        results.append({
            'fast': fast,
            'slow': slow,
            'signal': signal,
            'total_return': pf.total_return(),
            'sharpe': pf.sharpe_ratio(),
            'max_dd': pf.max_drawdown(),
            'num_trades': pf.trades.count()
        })

    # Convert to DataFrame
    results_df = pd.DataFrame(results)
    results_df = results_df.sort_values('sharpe', ascending=False)

    print("=" * 60)
    print("MACD PARAMETER OPTIMIZATION")
    print("=" * 60)
    print("\nTop 10 Configurations:")
    print(results_df.head(10).to_string(index=False))

    return results_df


# ==============================================================================
# 4. RSI + MACD WITH TREND FILTER
# ==============================================================================

def rsi_macd_with_trend_filter():
    """Add moving average trend filter"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate indicators
    rsi = vbt.RSI.run(price, window=14)
    macd = vbt.MACD.run(price, fast_window=12, slow_window=26, signal_window=9)
    ma_200 = vbt.MA.run(price, window=200)

    # Strategy 1: Without trend filter
    entries_no_filter = rsi.rsi_below(30) & macd.macd_above(macd.signal)
    exits_no_filter = rsi.rsi_above(70) | macd.macd_below(macd.signal)

    pf_no_filter = vbt.Portfolio.from_signals(
        price, entries_no_filter, exits_no_filter, fees=0.001
    )

    # Strategy 2: With trend filter (only long when above 200 MA)
    entries_with_filter = (
        rsi.rsi_below(30) &
        macd.macd_above(macd.signal) &
        (price > ma_200.ma)  # Trend filter
    )
    exits_with_filter = rsi.rsi_above(70) | macd.macd_below(macd.signal)

    pf_with_filter = vbt.Portfolio.from_signals(
        price, entries_with_filter, exits_with_filter, fees=0.001
    )

    print("=" * 60)
    print("RSI + MACD WITH TREND FILTER")
    print("=" * 60)

    print("\nWithout Trend Filter:")
    print(f"  Total Return: {pf_no_filter.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_no_filter.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_no_filter.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_no_filter.trades.win_rate():.2%}")
    print(f"  Trades: {pf_no_filter.trades.count()}")

    print("\nWith 200 MA Trend Filter:")
    print(f"  Total Return: {pf_with_filter.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_with_filter.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_with_filter.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_with_filter.trades.win_rate():.2%}")
    print(f"  Trades: {pf_with_filter.trades.count()}")

    return pf_no_filter, pf_with_filter


# ==============================================================================
# 5. ADVANCED: RSI DIVERGENCE DETECTION
# ==============================================================================

def rsi_divergence_strategy():
    """Detect RSI divergences for entries"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate RSI
    rsi = vbt.RSI.run(price, window=14)

    # Simple divergence detection
    # Bullish divergence: Price makes lower low, RSI makes higher low
    lookback = 14

    price_rolling_min = price.rolling(lookback).min()
    rsi_rolling_min = rsi.rsi.rolling(lookback).min()

    # Approximate divergence (simplified)
    bullish_div = (
        (price < price.shift(lookback)) &  # Lower price
        (rsi.rsi > rsi.rsi.shift(lookback)) &  # Higher RSI
        (rsi.rsi < 30)  # Still oversold
    )

    # Use divergence as entry
    macd = vbt.MACD.run(price, fast_window=12, slow_window=26, signal_window=9)

    entries = bullish_div & macd.macd_above(macd.signal)
    exits = rsi.rsi_above(70) | macd.macd_below(macd.signal)

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price, entries, exits, fees=0.001
    )

    print("=" * 60)
    print("RSI DIVERGENCE + MACD STRATEGY")
    print("=" * 60)
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    return pf


# ==============================================================================
# 6. MULTI-TIMEFRAME CONFIRMATION
# ==============================================================================

def multi_timeframe_rsi_macd():
    """Use higher timeframe for confirmation"""

    # Download daily data
    price_daily = vbt.YFData.download('BTC-USD', period='2y', interval='1d').get('Close')

    # Resample to weekly for higher timeframe
    price_weekly = price_daily.resample('W').agg({
        'Open': 'first',
        'High': 'max',
        'Low': 'min',
        'Close': 'last'
    })['Close']

    # Daily indicators
    rsi_daily = vbt.RSI.run(price_daily, window=14)
    macd_daily = vbt.MACD.run(price_daily, fast_window=12, slow_window=26, signal_window=9)

    # Weekly indicators
    macd_weekly = vbt.MACD.run(price_weekly, fast_window=12, slow_window=26, signal_window=9)

    # Resample weekly signal to daily
    weekly_bullish = macd_weekly.macd_above(macd_weekly.signal).reindex(
        price_daily.index, method='ffill'
    )

    # Entry: Daily oversold + Daily MACD bullish + Weekly MACD bullish
    entries = (
        rsi_daily.rsi_below(30) &
        macd_daily.macd_above(macd_daily.signal) &
        weekly_bullish
    )

    exits = rsi_daily.rsi_above(70) | macd_daily.macd_below(macd_daily.signal)

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price_daily, entries, exits, fees=0.001
    )

    print("=" * 60)
    print("MULTI-TIMEFRAME RSI + MACD")
    print("=" * 60)
    print("Daily: RSI(14), MACD(12/26/9)")
    print("Weekly: MACD(12/26/9) for confirmation")
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
    print("VECTORBT RSI + MACD EXAMPLES")
    print("="*60 + "\n")

    # 1. Basic strategy
    pf_basic = basic_rsi_macd()

    # 2. Optimize RSI thresholds
    results_rsi = optimize_rsi_thresholds()

    # 3. Optimize MACD parameters
    results_macd = optimize_macd_parameters()

    # 4. With trend filter
    pf_no_filter, pf_with_filter = rsi_macd_with_trend_filter()

    # 5. Divergence detection
    pf_divergence = rsi_divergence_strategy()

    # 6. Multi-timeframe
    pf_mtf = multi_timeframe_rsi_macd()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
