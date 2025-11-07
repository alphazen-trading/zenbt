"""
Moving Average Crossover Strategy - VectorBT Reference Implementation

Strategy Logic:
- Buy when fast MA crosses above slow MA
- Sell when fast MA crosses below slow MA

Features Demonstrated:
- Simple indicator calculation
- Signal generation with crossover detection
- Basic portfolio backtesting
- Parameter optimization

VectorBT Concepts Used:
- vbt.MA.run() - Moving average indicator
- ma_crossed_above() / ma_crossed_below() - Crossover detection
- Portfolio.from_signals() - Signal-based backtesting
- run_combs() - Parameter combinations
"""

import vectorbt as vbt
import pandas as pd
import numpy as np


# ==============================================================================
# 1. SIMPLE MOVING AVERAGE CROSSOVER (Single Asset, Single Parameter Set)
# ==============================================================================

def simple_ma_crossover():
    """Basic MA crossover with fixed parameters"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate indicators
    fast_ma = vbt.MA.run(price, window=10, short_name='fast')
    slow_ma = vbt.MA.run(price, window=50, short_name='slow')

    # Generate signals
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Run backtest
    pf = vbt.Portfolio.from_signals(
        close=price,
        entries=entries,
        exits=exits,
        init_cash=10_000,
        fees=0.001,  # 0.1% trading fee
        freq='1D'
    )

    # Display results
    print("=" * 60)
    print("SIMPLE MA CROSSOVER (10/50)")
    print("=" * 60)
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    # Plot equity curve
    pf.plot().show()

    return pf


# ==============================================================================
# 2. PARAMETER OPTIMIZATION (Testing Multiple Window Combinations)
# ==============================================================================

def optimize_ma_crossover():
    """Test multiple MA window combinations"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Define parameter space
    windows = np.arange(5, 101, 5)  # 5, 10, 15, ... 100

    # Run all combinations
    fast_ma, slow_ma = vbt.MA.run_combs(
        price,
        window=windows,
        r=2,  # Combinations of 2 windows
        short_names=['fast', 'slow']
    )

    # Generate signals for all combinations
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Backtest all combinations
    pf = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        size=np.inf,  # Use all available cash
        fees=0.001,
        freq='1D'
    )

    # Find best parameters
    sharpe_ratios = pf.sharpe_ratio()
    best_params = sharpe_ratios.idxmax()
    best_sharpe = sharpe_ratios.max()

    print("=" * 60)
    print("MA CROSSOVER OPTIMIZATION")
    print("=" * 60)
    print(f"Tested {len(windows) * (len(windows) - 1) // 2} combinations")
    print(f"Best Parameters: Fast={best_params[0]}, Slow={best_params[1]}")
    print(f"Best Sharpe Ratio: {best_sharpe:.2f}")

    # Show stats for best strategy
    best_pf = pf[best_params]
    print(f"\nBest Strategy Performance:")
    print(f"Total Return: {best_pf.total_return():.2%}")
    print(f"Max Drawdown: {best_pf.max_drawdown():.2%}")
    print(f"Win Rate: {best_pf.trades.win_rate():.2%}")

    # Visualize as heatmap
    fig = sharpe_ratios.vbt.heatmap(
        x_level='fast_window',
        y_level='slow_window',
        trace_kwargs=dict(
            colorscale='RdYlGn',
            zmid=0
        )
    )
    fig.show()

    return pf, best_params


# ==============================================================================
# 3. MULTI-ASSET MA CROSSOVER
# ==============================================================================

def multi_asset_ma_crossover():
    """Apply MA crossover to multiple cryptocurrencies"""

    # Download multiple assets
    symbols = ['BTC-USD', 'ETH-USD', 'LTC-USD', 'XRP-USD']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate indicators
    fast_ma = vbt.MA.run(price, window=10)
    slow_ma = vbt.MA.run(price, window=50)

    # Generate signals
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Backtest each asset
    pf = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        size=np.inf,
        fees=0.001,
        freq='1D'
    )

    # Compare performance across assets
    print("=" * 60)
    print("MULTI-ASSET MA CROSSOVER")
    print("=" * 60)

    for symbol in symbols:
        symbol_pf = pf[symbol]
        print(f"\n{symbol}:")
        print(f"  Total Return: {symbol_pf.total_return():.2%}")
        print(f"  Sharpe Ratio: {symbol_pf.sharpe_ratio():.2f}")
        print(f"  Max Drawdown: {symbol_pf.max_drawdown():.2%}")
        print(f"  Win Rate: {symbol_pf.trades.win_rate():.2%}")

    # Plot all equity curves
    pf.plot().show()

    return pf


# ==============================================================================
# 4. MA CROSSOVER WITH STOP LOSS AND TAKE PROFIT
# ==============================================================================

def ma_crossover_with_stops():
    """MA crossover with risk management via stops"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate indicators
    fast_ma = vbt.MA.run(price, window=10)
    slow_ma = vbt.MA.run(price, window=50)

    # Generate signals
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Backtest WITHOUT stops
    pf_no_stops = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        size=np.inf,
        fees=0.001,
        freq='1D'
    )

    # Backtest WITH stops
    pf_with_stops = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        sl_stop=0.05,  # 5% stop loss
        tp_stop=0.10,  # 10% take profit
        size=np.inf,
        fees=0.001,
        freq='1D'
    )

    print("=" * 60)
    print("MA CROSSOVER WITH/WITHOUT STOPS")
    print("=" * 60)

    print("\nWithout Stops:")
    print(f"  Total Return: {pf_no_stops.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_no_stops.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_no_stops.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_no_stops.trades.win_rate():.2%}")

    print("\nWith 5% SL / 10% TP:")
    print(f"  Total Return: {pf_with_stops.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_with_stops.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_with_stops.max_drawdown():.2%}")
    print(f"  Win Rate: {pf_with_stops.trades.win_rate():.2%}")
    print(f"  Avg Win: {pf_with_stops.trades.winning.avg_return():.2%}")
    print(f"  Avg Loss: {pf_with_stops.trades.losing.avg_return():.2%}")

    return pf_no_stops, pf_with_stops


# ==============================================================================
# 5. WALK-FORWARD ANALYSIS
# ==============================================================================

def walk_forward_ma_crossover():
    """Walk-forward optimization to prevent overfitting"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='3y').get('Close')

    # Define splits (6 months train, 1 month test, rolling)
    splits = vbt.Splitter.from_rolling(
        price.index,
        window_len=180,  # 6 months train
        set_lens=(180, 30),  # Train: 6 months, Test: 1 month
        offset=30  # Roll forward 1 month
    )

    results = []

    for i, (train_indices, test_indices) in enumerate(splits):
        # Get train/test data
        train_price = price.iloc[train_indices]
        test_price = price.iloc[test_indices]

        # Optimize on training data
        windows = np.arange(5, 100, 5)
        fast_ma, slow_ma = vbt.MA.run_combs(train_price, window=windows, r=2)
        entries = fast_ma.ma_crossed_above(slow_ma)
        exits = fast_ma.ma_crossed_below(slow_ma)

        pf_train = vbt.Portfolio.from_signals(
            train_price, entries, exits, fees=0.001
        )

        # Find best parameters
        best_params = pf_train.sharpe_ratio().idxmax()

        # Test on out-of-sample data
        fast_ma_test = vbt.MA.run(test_price, best_params[0])
        slow_ma_test = vbt.MA.run(test_price, best_params[1])
        entries_test = fast_ma_test.ma_crossed_above(slow_ma_test)
        exits_test = fast_ma_test.ma_crossed_below(slow_ma_test)

        pf_test = vbt.Portfolio.from_signals(
            test_price, entries_test, exits_test, fees=0.001
        )

        # Store results
        results.append({
            'split': i,
            'best_fast': best_params[0],
            'best_slow': best_params[1],
            'train_sharpe': pf_train[best_params].sharpe_ratio(),
            'test_sharpe': pf_test.sharpe_ratio(),
            'test_return': pf_test.total_return()
        })

    # Analyze results
    results_df = pd.DataFrame(results)

    print("=" * 60)
    print("WALK-FORWARD ANALYSIS")
    print("=" * 60)
    print(f"\nTested {len(results)} splits")
    print(f"Avg In-Sample Sharpe: {results_df['train_sharpe'].mean():.2f}")
    print(f"Avg Out-of-Sample Sharpe: {results_df['test_sharpe'].mean():.2f}")
    print(f"Avg OOS Return: {results_df['test_return'].mean():.2%}")
    print(f"\nParameter Stability:")
    print(f"  Fast MA: {results_df['best_fast'].mean():.1f} ± {results_df['best_fast'].std():.1f}")
    print(f"  Slow MA: {results_df['best_slow'].mean():.1f} ± {results_df['best_slow'].std():.1f}")

    print("\nDetailed Results:")
    print(results_df.to_string(index=False))

    return results_df


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    # Run all examples
    print("\n" + "="*60)
    print("VECTORBT MA CROSSOVER EXAMPLES")
    print("="*60 + "\n")

    # 1. Simple strategy
    pf_simple = simple_ma_crossover()

    # 2. Parameter optimization
    pf_opt, best_params = optimize_ma_crossover()

    # 3. Multi-asset
    pf_multi = multi_asset_ma_crossover()

    # 4. With stops
    pf_no_stops, pf_with_stops = ma_crossover_with_stops()

    # 5. Walk-forward
    wf_results = walk_forward_ma_crossover()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
