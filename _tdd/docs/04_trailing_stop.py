"""
Trailing Stop Strategy - VectorBT Reference Implementation

Strategy Logic:
- Enter on simple signal (MA crossover, RSI, etc.)
- Use trailing stop to lock in profits
- Compare fixed stop vs trailing stop

Features Demonstrated:
- Stop loss management
- Trailing stops
- Take profit levels
- Risk/reward optimization

VectorBT Concepts Used:
- Portfolio.from_signals() with sl_stop, tp_stop
- sl_trail parameter for trailing stops
- Stop distance modes (percentage, ATR-based)
"""

import vectorbt as vbt
import pandas as pd
import numpy as np


# ==============================================================================
# 1. BASIC TRAILING STOP
# ==============================================================================

def basic_trailing_stop():
    """Compare fixed vs trailing stop loss"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Simple MA crossover for entries
    fast_ma = vbt.MA.run(price, window=10)
    slow_ma = vbt.MA.run(price, window=50)

    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Strategy 1: No stops
    pf_no_stop = vbt.Portfolio.from_signals(
        price, entries, exits, fees=0.001
    )

    # Strategy 2: Fixed 5% stop loss
    pf_fixed_stop = vbt.Portfolio.from_signals(
        price,
        entries, exits,
        sl_stop=0.05,  # 5% stop loss
        fees=0.001
    )

    # Strategy 3: Trailing 5% stop
    pf_trailing_stop = vbt.Portfolio.from_signals(
        price,
        entries, exits,
        sl_stop=0.05,   # Initial 5% stop
        sl_trail=True,  # Enable trailing
        fees=0.001
    )

    print("=" * 60)
    print("STOP LOSS COMPARISON")
    print("=" * 60)

    print("\nNo Stop Loss:")
    print(f"  Total Return: {pf_no_stop.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_no_stop.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_no_stop.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_no_stop.trades.win_rate():.2%}")

    print("\nFixed 5% Stop Loss:")
    print(f"  Total Return: {pf_fixed_stop.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_fixed_stop.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_fixed_stop.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_fixed_stop.trades.win_rate():.2%}")
    print(f"  Avg Win: {pf_fixed_stop.trades.winning.avg_return():.2%}")
    print(f"  Avg Loss: {pf_fixed_stop.trades.losing.avg_return():.2%}")

    print("\nTrailing 5% Stop:")
    print(f"  Total Return: {pf_trailing_stop.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_trailing_stop.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_trailing_stop.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_trailing_stop.trades.win_rate():.2%}")
    print(f"  Avg Win: {pf_trailing_stop.trades.winning.avg_return():.2%}")
    print(f"  Avg Loss: {pf_trailing_stop.trades.losing.avg_return():.2%}")

    return pf_no_stop, pf_fixed_stop, pf_trailing_stop


# ==============================================================================
# 2. OPTIMIZE TRAILING STOP DISTANCE
# ==============================================================================

def optimize_trailing_stop_distance():
    """Test different trailing stop percentages"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='2y').get('Close')

    # Generate simple signals (RSI)
    rsi = vbt.RSI.run(price, window=14)
    entries = rsi.rsi_below(30)
    exits = rsi.rsi_above(70)

    # Test different stop distances
    stop_distances = np.arange(0.02, 0.21, 0.02)  # 2% to 20%

    results = []

    for stop_pct in stop_distances:
        # Fixed stop
        pf_fixed = vbt.Portfolio.from_signals(
            price, entries, exits,
            sl_stop=stop_pct,
            fees=0.001
        )

        # Trailing stop
        pf_trailing = vbt.Portfolio.from_signals(
            price, entries, exits,
            sl_stop=stop_pct,
            sl_trail=True,
            fees=0.001
        )

        results.append({
            'stop_pct': stop_pct * 100,
            'fixed_return': pf_fixed.total_return(),
            'fixed_sharpe': pf_fixed.sharpe_ratio(),
            'fixed_max_dd': pf_fixed.max_drawdown(),
            'trailing_return': pf_trailing.total_return(),
            'trailing_sharpe': pf_trailing.sharpe_ratio(),
            'trailing_max_dd': pf_trailing.max_drawdown(),
        })

    # Convert to DataFrame
    results_df = pd.DataFrame(results)

    print("=" * 60)
    print("TRAILING STOP DISTANCE OPTIMIZATION")
    print("=" * 60)
    print("\nResults:")
    print(results_df.to_string(index=False))

    # Find best
    best_trailing_idx = results_df['trailing_sharpe'].idxmax()
    best_trailing = results_df.iloc[best_trailing_idx]

    print(f"\nBest Trailing Stop:")
    print(f"  Distance: {best_trailing['stop_pct']:.0f}%")
    print(f"  Sharpe: {best_trailing['trailing_sharpe']:.2f}")
    print(f"  Return: {best_trailing['trailing_return']:.2%}")

    return results_df


# ==============================================================================
# 3. STOP LOSS + TAKE PROFIT COMBINATION
# ==============================================================================

def stop_and_target_optimization():
    """Optimize both stop loss and take profit levels"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Simple entry signal
    fast_ma = vbt.MA.run(price, 10)
    slow_ma = vbt.MA.run(price, 50)
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Test different SL/TP combinations
    stop_losses = [0.03, 0.05, 0.07, 0.10]  # 3%, 5%, 7%, 10%
    take_profits = [0.05, 0.10, 0.15, 0.20]  # 5%, 10%, 15%, 20%

    results = []

    for sl in stop_losses:
        for tp in take_profits:
            pf = vbt.Portfolio.from_signals(
                price, entries, exits,
                sl_stop=sl,
                tp_stop=tp,
                sl_trail=True,  # Trailing stop
                fees=0.001
            )

            results.append({
                'sl_pct': sl * 100,
                'tp_pct': tp * 100,
                'risk_reward': tp / sl,
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
    print("STOP LOSS + TAKE PROFIT OPTIMIZATION")
    print("=" * 60)
    print("\nTop 10 Configurations:")
    print(results_df.head(10).to_string(index=False))

    # Best configuration
    best = results_df.iloc[0]
    print(f"\nBest Configuration:")
    print(f"  Stop Loss: {best['sl_pct']:.0f}%")
    print(f"  Take Profit: {best['tp_pct']:.0f}%")
    print(f"  Risk/Reward: {best['risk_reward']:.2f}")
    print(f"  Sharpe: {best['sharpe']:.2f}")
    print(f"  Win Rate: {best['win_rate']:.2%}")

    return results_df


# ==============================================================================
# 4. ATR-BASED TRAILING STOP
# ==============================================================================

def atr_based_trailing_stop():
    """Use ATR (Average True Range) for dynamic stop distance"""

    # Download OHLC data
    data = vbt.YFData.download('BTC-USD', period='2y')
    close = data.get('Close')
    high = data.get('High')
    low = data.get('Low')

    # Calculate ATR
    atr = vbt.ATR.run(high, low, close, window=14)

    # Entry signal (simple MA cross)
    fast_ma = vbt.MA.run(close, 10)
    slow_ma = vbt.MA.run(close, 50)
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Note: VectorBT doesn't directly support ATR-based stops in from_signals
    # This would require from_order_func for full implementation
    # Here we approximate with fixed percentage

    # Strategy 1: Fixed 5% stop
    pf_fixed = vbt.Portfolio.from_signals(
        close, entries, exits,
        sl_stop=0.05,
        sl_trail=True,
        fees=0.001
    )

    # Strategy 2: Approximate ATR-based (2x ATR as percentage)
    # Calculate average ATR as percentage of price
    atr_pct = (atr.atr / close).mean()
    atr_stop = atr_pct * 2  # 2x ATR

    pf_atr = vbt.Portfolio.from_signals(
        close, entries, exits,
        sl_stop=atr_stop,
        sl_trail=True,
        fees=0.001
    )

    print("=" * 60)
    print("ATR-BASED TRAILING STOP")
    print("=" * 60)
    print(f"Average ATR: {atr_pct:.2%} of price")
    print(f"Stop Distance (2x ATR): {atr_stop:.2%}")

    print("\nFixed 5% Stop:")
    print(f"  Total Return: {pf_fixed.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_fixed.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_fixed.max_drawdown():.2%}")

    print(f"\nATR-Based Stop ({atr_stop:.2%}):")
    print(f"  Total Return: {pf_atr.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_atr.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_atr.max_drawdown():.2%}")

    return pf_fixed, pf_atr


# ==============================================================================
# 5. CHANDELIER EXIT (ATR-BASED TRAILING STOP)
# ==============================================================================

def chandelier_exit_strategy():
    """Chandelier Exit: Trail stop below highest high by ATR multiple"""

    # Download data
    data = vbt.YFData.download('ETH-USD', period='2y')
    close = data.get('Close')
    high = data.get('High')
    low = data.get('Low')

    # Calculate ATR
    atr = vbt.ATR.run(high, low, close, window=14)

    # Entry: Simple MA cross
    fast_ma = vbt.MA.run(close, 10)
    slow_ma = vbt.MA.run(close, 50)
    entries = fast_ma.ma_crossed_above(slow_ma)

    # Chandelier Exit: Highest High - (ATR × Multiplier)
    # Approximate with fixed percentage based on average ATR
    atr_multiplier = 3
    chandelier_pct = (atr.atr / close).mean() * atr_multiplier

    # Use as trailing stop
    pf = vbt.Portfolio.from_signals(
        close,
        entries=entries,
        exits=False,  # No signal exit, only stops
        sl_stop=chandelier_pct,
        sl_trail=True,
        fees=0.001
    )

    print("=" * 60)
    print("CHANDELIER EXIT STRATEGY")
    print("=" * 60)
    print(f"ATR Period: 14")
    print(f"ATR Multiplier: {atr_multiplier}")
    print(f"Stop Distance: {chandelier_pct:.2%}")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Avg Win: {pf.trades.winning.avg_return():.2%}")

    return pf


# ==============================================================================
# 6. MULTI-LEVEL PROFIT TARGETS
# ==============================================================================

def multi_level_profit_targets():
    """Scale out at multiple profit levels"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Entry signal
    rsi = vbt.RSI.run(price, 14)
    entries = rsi.rsi_below(30)

    # Compare different exit strategies
    # 1. Single exit at 10%
    pf_single = vbt.Portfolio.from_signals(
        price, entries, False,
        tp_stop=0.10,
        sl_stop=0.05,
        fees=0.001
    )

    # 2. Single exit at 20%
    pf_higher = vbt.Portfolio.from_signals(
        price, entries, False,
        tp_stop=0.20,
        sl_stop=0.05,
        fees=0.001
    )

    # 3. Trailing stop (no TP)
    pf_trailing = vbt.Portfolio.from_signals(
        price, entries, False,
        sl_stop=0.05,
        sl_trail=True,
        fees=0.001
    )

    print("=" * 60)
    print("PROFIT TARGET STRATEGIES")
    print("=" * 60)

    print("\n10% Take Profit:")
    print(f"  Total Return: {pf_single.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_single.sharpe_ratio():.2f}")
    print(f"  Win Rate: {pf_single.trades.win_rate():.2%}")

    print("\n20% Take Profit:")
    print(f"  Total Return: {pf_higher.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_higher.sharpe_ratio():.2f}")
    print(f"  Win Rate: {pf_higher.trades.win_rate():.2%}")

    print("\nTrailing Stop (No TP):")
    print(f"  Total Return: {pf_trailing.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_trailing.sharpe_ratio():.2f}")
    print(f"  Win Rate: {pf_trailing.trades.win_rate():.2%}")

    return pf_single, pf_higher, pf_trailing


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    print("\n" + "="*60)
    print("VECTORBT TRAILING STOP EXAMPLES")
    print("="*60 + "\n")

    # 1. Basic comparison
    pf_no, pf_fixed, pf_trailing = basic_trailing_stop()

    # 2. Optimize stop distance
    results_stop = optimize_trailing_stop_distance()

    # 3. SL + TP optimization
    results_sl_tp = stop_and_target_optimization()

    # 4. ATR-based stops
    pf_fixed_atr, pf_atr = atr_based_trailing_stop()

    # 5. Chandelier exit
    pf_chandelier = chandelier_exit_strategy()

    # 6. Multi-level targets
    pf_10, pf_20, pf_trail = multi_level_profit_targets()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
