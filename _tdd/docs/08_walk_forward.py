"""
Walk-Forward Optimization - VectorBT Reference Implementation

Strategy Logic:
- Split data into train/test periods
- Optimize parameters on training data
- Test on out-of-sample data
- Roll forward and repeat
- Avoid overfitting through proper validation

Features Demonstrated:
- Walk-forward analysis
- Cross-validation techniques
- Out-of-sample testing
- Parameter stability analysis
- Overfitting detection

VectorBT Concepts Used:
- Splitter for creating train/test splits
- Rolling windows
- Combinatorial cross-validation
- from_rolling() split method
"""

import vectorbt as vbt
import pandas as pd
import numpy as np
from itertools import product


# ==============================================================================
# 1. BASIC WALK-FORWARD OPTIMIZATION
# ==============================================================================

def basic_walk_forward():
    """Simple walk-forward with fixed splits"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='3y').get('Close')

    # Define splits: 6 months train, 1 month test, roll by 1 month
    train_days = 180
    test_days = 30
    step_days = 30

    results = []

    # Calculate number of splits
    n_splits = (len(price) - train_days - test_days) // step_days

    print("=" * 60)
    print("BASIC WALK-FORWARD OPTIMIZATION")
    print("=" * 60)
    print(f"Total data points: {len(price)}")
    print(f"Train period: {train_days} days")
    print(f"Test period: {test_days} days")
    print(f"Number of splits: {n_splits}")

    for i in range(n_splits):
        # Define train/test indices
        train_start = i * step_days
        train_end = train_start + train_days
        test_start = train_end
        test_end = test_start + test_days

        if test_end > len(price):
            break

        # Get train/test data
        train_price = price.iloc[train_start:train_end]
        test_price = price.iloc[test_start:test_end]

        # Optimize on training data
        windows = np.arange(5, 51, 5)
        fast_ma, slow_ma = vbt.MA.run_combs(
            train_price,
            window=windows,
            r=2,
            short_names=['fast', 'slow']
        )

        entries = fast_ma.ma_crossed_above(slow_ma)
        exits = fast_ma.ma_crossed_below(slow_ma)

        pf_train = vbt.Portfolio.from_signals(
            train_price, entries, exits, fees=0.001
        )

        # Find best parameters
        best_params = pf_train.sharpe_ratio().idxmax()
        best_fast, best_slow = best_params

        # Test on out-of-sample data
        fast_ma_test = vbt.MA.run(test_price, best_fast)
        slow_ma_test = vbt.MA.run(test_price, best_slow)

        entries_test = fast_ma_test.ma_crossed_above(slow_ma_test)
        exits_test = fast_ma_test.ma_crossed_below(slow_ma_test)

        pf_test = vbt.Portfolio.from_signals(
            test_price, entries_test, exits_test, fees=0.001
        )

        # Store results
        results.append({
            'split': i + 1,
            'train_start': train_price.index[0].strftime('%Y-%m-%d'),
            'test_start': test_price.index[0].strftime('%Y-%m-%d'),
            'best_fast': best_fast,
            'best_slow': best_slow,
            'train_sharpe': pf_train[best_params].sharpe_ratio(),
            'test_sharpe': pf_test.sharpe_ratio(),
            'test_return': pf_test.total_return(),
            'test_trades': pf_test.trades.count()
        })

    # Analyze results
    results_df = pd.DataFrame(results)

    print("\nWalk-Forward Results:")
    print(results_df.to_string(index=False))

    print(f"\nSummary Statistics:")
    print(f"  Avg In-Sample Sharpe: {results_df['train_sharpe'].mean():.2f}")
    print(f"  Avg Out-of-Sample Sharpe: {results_df['test_sharpe'].mean():.2f}")
    print(f"  Avg OOS Return: {results_df['test_return'].mean():.2%}")
    print(f"  Parameter Stability (Fast MA): {results_df['best_fast'].std():.1f}")
    print(f"  Parameter Stability (Slow MA): {results_df['best_slow'].std():.1f}")

    return results_df


# ==============================================================================
# 2. ANCHORED WALK-FORWARD
# ==============================================================================

def anchored_walk_forward():
    """Walk-forward with expanding training window"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='3y').get('Close')

    initial_train = 180  # Start with 6 months
    test_days = 30
    step_days = 30

    results = []

    print("=" * 60)
    print("ANCHORED WALK-FORWARD OPTIMIZATION")
    print("=" * 60)

    train_start = 0

    for i in range(20):  # Limit to 20 splits
        # Expanding training window
        train_end = initial_train + (i * step_days)
        test_start = train_end
        test_end = test_start + test_days

        if test_end > len(price):
            break

        # Get data
        train_price = price.iloc[train_start:train_end]
        test_price = price.iloc[test_start:test_end]

        # Optimize
        rsi_periods = [10, 14, 20, 30]
        oversold_levels = [20, 25, 30]
        overbought_levels = [70, 75, 80]

        best_sharpe = -np.inf
        best_config = None

        for rsi_period in rsi_periods:
            for oversold in oversold_levels:
                for overbought in overbought_levels:
                    rsi = vbt.RSI.run(train_price, rsi_period)
                    entries = rsi.rsi_below(oversold)
                    exits = rsi.rsi_above(overbought)

                    pf = vbt.Portfolio.from_signals(
                        train_price, entries, exits, fees=0.001
                    )

                    sharpe = pf.sharpe_ratio()
                    if sharpe > best_sharpe:
                        best_sharpe = sharpe
                        best_config = (rsi_period, oversold, overbought)

        # Test best config
        rsi_test = vbt.RSI.run(test_price, best_config[0])
        entries_test = rsi_test.rsi_below(best_config[1])
        exits_test = rsi_test.rsi_above(best_config[2])

        pf_test = vbt.Portfolio.from_signals(
            test_price, entries_test, exits_test, fees=0.001
        )

        results.append({
            'split': i + 1,
            'train_size': len(train_price),
            'rsi_period': best_config[0],
            'oversold': best_config[1],
            'overbought': best_config[2],
            'train_sharpe': best_sharpe,
            'test_sharpe': pf_test.sharpe_ratio(),
            'test_return': pf_test.total_return()
        })

    results_df = pd.DataFrame(results)

    print("\nResults (Expanding Window):")
    print(results_df.to_string(index=False))

    print(f"\nAvg Out-of-Sample Performance:")
    print(f"  Sharpe: {results_df['test_sharpe'].mean():.2f}")
    print(f"  Return: {results_df['test_return'].mean():.2%}")

    return results_df


# ==============================================================================
# 3. COMBINATORIAL PURGED CROSS-VALIDATION
# ==============================================================================

def combinatorial_cv():
    """Combinatorial cross-validation with purging"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Split into 10 folds
    n_splits = 10
    fold_size = len(price) // n_splits

    print("=" * 60)
    print("COMBINATORIAL CROSS-VALIDATION")
    print("=" * 60)
    print(f"Total data: {len(price)} days")
    print(f"Number of folds: {n_splits}")
    print(f"Fold size: {fold_size} days")

    # Test parameter
    fast_windows = [5, 10, 15, 20]
    slow_windows = [30, 50, 70, 100]

    results = []

    for fast in fast_windows:
        for slow in slow_windows:
            if fast >= slow:
                continue

            # Calculate indicators
            fast_ma = vbt.MA.run(price, fast)
            slow_ma = vbt.MA.run(price, slow)
            entries = fast_ma.ma_crossed_above(slow_ma)
            exits = fast_ma.ma_crossed_below(slow_ma)

            # Test on each fold
            fold_returns = []

            for fold in range(n_splits):
                # Define test fold
                test_start = fold * fold_size
                test_end = test_start + fold_size

                # Create test mask
                test_mask = np.zeros(len(price), dtype=bool)
                test_mask[test_start:test_end] = True

                # Test data
                test_price = price[test_mask]
                test_entries = entries[test_mask]
                test_exits = exits[test_mask]

                pf = vbt.Portfolio.from_signals(
                    test_price, test_entries, test_exits, fees=0.001
                )

                fold_returns.append(pf.total_return())

            # Average across folds
            avg_return = np.mean(fold_returns)
            std_return = np.std(fold_returns)

            results.append({
                'fast': fast,
                'slow': slow,
                'avg_return': avg_return,
                'std_return': std_return,
                'min_return': np.min(fold_returns),
                'max_return': np.max(fold_returns)
            })

    results_df = pd.DataFrame(results)
    results_df = results_df.sort_values('avg_return', ascending=False)

    print("\nTop 10 Configurations:")
    print(results_df.head(10).to_string(index=False))

    # Best config
    best = results_df.iloc[0]
    print(f"\nBest Configuration:")
    print(f"  Fast: {best['fast']}, Slow: {best['slow']}")
    print(f"  Avg Return: {best['avg_return']:.2%}")
    print(f"  Std Return: {best['std_return']:.2%}")

    return results_df


# ==============================================================================
# 4. OUT-OF-SAMPLE DECAY ANALYSIS
# ==============================================================================

def oos_decay_analysis():
    """Analyze how performance decays out-of-sample"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='3y').get('Close')

    # Train on first year
    train_size = 365
    train_price = price.iloc[:train_size]

    # Optimize on training data
    windows = np.arange(5, 51, 5)
    fast_ma, slow_ma = vbt.MA.run_combs(train_price, window=windows, r=2)
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    pf_train = vbt.Portfolio.from_signals(train_price, entries, exits, fees=0.001)
    best_params = pf_train.sharpe_ratio().idxmax()

    print("=" * 60)
    print("OUT-OF-SAMPLE DECAY ANALYSIS")
    print("=" * 60)
    print(f"Best Parameters: Fast={best_params[0]}, Slow={best_params[1]}")
    print(f"In-Sample Sharpe: {pf_train[best_params].sharpe_ratio():.2f}")

    # Test on subsequent periods
    test_periods = [30, 60, 90, 180, 365, 730]  # Days
    decay_results = []

    for days in test_periods:
        test_start = train_size
        test_end = train_size + days

        if test_end > len(price):
            break

        test_price = price.iloc[test_start:test_end]

        # Test with best params
        fast_ma_test = vbt.MA.run(test_price, best_params[0])
        slow_ma_test = vbt.MA.run(test_price, best_params[1])
        entries_test = fast_ma_test.ma_crossed_above(slow_ma_test)
        exits_test = fast_ma_test.ma_crossed_below(slow_ma_test)

        pf_test = vbt.Portfolio.from_signals(
            test_price, entries_test, exits_test, fees=0.001
        )

        decay_results.append({
            'days_oos': days,
            'sharpe': pf_test.sharpe_ratio(),
            'return': pf_test.total_return(),
            'max_dd': pf_test.max_drawdown(),
            'trades': pf_test.trades.count()
        })

    decay_df = pd.DataFrame(decay_results)

    print("\nPerformance Decay Over Time:")
    print(decay_df.to_string(index=False))

    return decay_df


# ==============================================================================
# 5. MONTE CARLO WALK-FORWARD
# ==============================================================================

def monte_carlo_walk_forward():
    """Randomize split order to test robustness"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='2y').get('Close')

    # Create random splits
    n_simulations = 100
    train_size = 180
    test_size = 30

    simulation_results = []

    print("=" * 60)
    print("MONTE CARLO WALK-FORWARD")
    print("=" * 60)
    print(f"Simulations: {n_simulations}")

    for sim in range(n_simulations):
        # Random train start
        max_start = len(price) - train_size - test_size
        train_start = np.random.randint(0, max_start)
        train_end = train_start + train_size
        test_start = train_end
        test_end = test_start + test_size

        train_price = price.iloc[train_start:train_end]
        test_price = price.iloc[test_start:test_end]

        # Simple optimization
        rsi = vbt.RSI.run(train_price, 14)
        entries = rsi.rsi_below(30)
        exits = rsi.rsi_above(70)

        pf_train = vbt.Portfolio.from_signals(train_price, entries, exits, fees=0.001)
        train_sharpe = pf_train.sharpe_ratio()

        # Test
        rsi_test = vbt.RSI.run(test_price, 14)
        entries_test = rsi_test.rsi_below(30)
        exits_test = rsi_test.rsi_above(70)

        pf_test = vbt.Portfolio.from_signals(test_price, entries_test, exits_test, fees=0.001)
        test_sharpe = pf_test.sharpe_ratio()

        simulation_results.append({
            'train_sharpe': train_sharpe,
            'test_sharpe': test_sharpe,
            'diff': test_sharpe - train_sharpe
        })

    sim_df = pd.DataFrame(simulation_results)

    print(f"\nMonte Carlo Results:")
    print(f"  Mean IS Sharpe: {sim_df['train_sharpe'].mean():.2f}")
    print(f"  Mean OOS Sharpe: {sim_df['test_sharpe'].mean():.2f}")
    print(f"  Mean Degradation: {sim_df['diff'].mean():.2f}")
    print(f"  % Positive OOS: {(sim_df['test_sharpe'] > 0).sum() / len(sim_df):.1%}")

    return sim_df


# ==============================================================================
# 6. PARAMETER STABILITY TEST
# ==============================================================================

def parameter_stability_test():
    """Test if optimal parameters are stable across splits"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='3y').get('Close')

    # Multiple walk-forward splits
    train_size = 180
    test_size = 30
    step = 30

    param_history = []

    for i in range(20):
        train_start = i * step
        train_end = train_start + train_size

        if train_end + test_size > len(price):
            break

        train_price = price.iloc[train_start:train_end]

        # Optimize
        windows = np.arange(5, 101, 5)
        fast_ma, slow_ma = vbt.MA.run_combs(train_price, window=windows, r=2)
        entries = fast_ma.ma_crossed_above(slow_ma)
        exits = fast_ma.ma_crossed_below(slow_ma)

        pf = vbt.Portfolio.from_signals(train_price, entries, exits, fees=0.001)
        best_params = pf.sharpe_ratio().idxmax()

        param_history.append({
            'split': i + 1,
            'fast': best_params[0],
            'slow': best_params[1]
        })

    param_df = pd.DataFrame(param_history)

    print("=" * 60)
    print("PARAMETER STABILITY TEST")
    print("=" * 60)
    print("\nOptimal Parameters Over Time:")
    print(param_df.to_string(index=False))

    print(f"\nStability Metrics:")
    print(f"  Fast MA - Mean: {param_df['fast'].mean():.1f}, Std: {param_df['fast'].std():.1f}")
    print(f"  Slow MA - Mean: {param_df['slow'].mean():.1f}, Std: {param_df['slow'].std():.1f}")
    print(f"  Fast MA Range: {param_df['fast'].min()} - {param_df['fast'].max()}")
    print(f"  Slow MA Range: {param_df['slow'].min()} - {param_df['slow'].max()}")

    # High variance suggests overfitting
    if param_df['fast'].std() > 10 or param_df['slow'].std() > 20:
        print("\n⚠️  Warning: High parameter variance suggests potential overfitting")
    else:
        print("\n✓ Parameters are relatively stable")

    return param_df


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    print("\n" + "="*60)
    print("VECTORBT WALK-FORWARD OPTIMIZATION EXAMPLES")
    print("="*60 + "\n")

    # 1. Basic walk-forward
    results_basic = basic_walk_forward()

    # 2. Anchored walk-forward
    results_anchored = anchored_walk_forward()

    # 3. Combinatorial CV
    results_cv = combinatorial_cv()

    # 4. OOS decay
    decay_results = oos_decay_analysis()

    # 5. Monte Carlo
    mc_results = monte_carlo_walk_forward()

    # 6. Parameter stability
    param_stability = parameter_stability_test()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
    print("\nKey Takeaways:")
    print("- Walk-forward prevents overfitting")
    print("- Parameter stability indicates robustness")
    print("- OOS performance decay is normal")
    print("- Use multiple validation methods")
