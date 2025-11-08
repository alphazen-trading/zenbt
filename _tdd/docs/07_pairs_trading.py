"""
Pairs Trading Strategy - VectorBT Reference Implementation

Strategy Logic:
- Find cointegrated asset pairs
- Trade the spread between them
- Buy spread when z-score < -2 (oversold)
- Sell spread when z-score > 2 (overbought)
- Exit when z-score crosses 0 (mean reversion)

Features Demonstrated:
- Cointegration testing
- Spread calculation
- Z-score trading
- Market neutral strategy
- Statistical arbitrage

VectorBT Concepts Used:
- Multi-asset operations
- Spread trading
- Custom indicators
- Portfolio.from_order_func() for pairs logic
"""

import vectorbt as vbt
import pandas as pd
import numpy as np
from statsmodels.tsa.stattools import coint
from numba import njit


# ==============================================================================
# 1. FIND COINTEGRATED PAIRS
# ==============================================================================

def find_cointegrated_pairs():
    """Test multiple pairs for cointegration"""

    # Download potential pairs
    symbols = ['GLD', 'GDX',    # Gold ETF and Gold Miners
               'SPY', 'QQQ',    # S&P 500 and NASDAQ
               'XLE', 'XOP',    # Energy sector ETFs
               'BTC-USD', 'ETH-USD',  # Crypto
               'USO', 'XLE']    # Oil and Energy

    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    print("=" * 60)
    print("COINTEGRATION ANALYSIS")
    print("=" * 60)

    # Test all pairs
    cointegrated_pairs = []

    for i in range(len(symbols)):
        for j in range(i+1, len(symbols)):
            asset1 = symbols[i]
            asset2 = symbols[j]

            # Test cointegration
            score, pvalue, _ = coint(price[asset1], price[asset2])

            cointegrated_pairs.append({
                'asset1': asset1,
                'asset2': asset2,
                'pvalue': pvalue,
                'cointegrated': pvalue < 0.05
            })

    # Convert to DataFrame
    pairs_df = pd.DataFrame(cointegrated_pairs)
    pairs_df = pairs_df.sort_values('pvalue')

    print("\nCointegrated Pairs (p-value < 0.05):")
    cointegrated = pairs_df[pairs_df['cointegrated']]
    print(cointegrated.to_string(index=False))

    print("\nBest Candidate Pairs:")
    print(pairs_df.head(5).to_string(index=False))

    return pairs_df, price


# ==============================================================================
# 2. BASIC PAIRS TRADING STRATEGY
# ==============================================================================

def basic_pairs_trading():
    """Simple pairs trading on GLD/GDX"""

    # Download data
    symbols = ['GLD', 'GDX']  # Gold ETF and Gold Miners
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Test cointegration
    score, pvalue, _ = coint(price['GLD'], price['GDX'])
    print("=" * 60)
    print("PAIRS TRADING: GLD vs GDX")
    print("=" * 60)
    print(f"Cointegration p-value: {pvalue:.4f}")
    print(f"Cointegrated: {pvalue < 0.05}")

    if pvalue >= 0.05:
        print("Warning: Pair not cointegrated!")

    # Calculate spread
    spread = price['GLD'] - price['GDX']

    # Calculate z-score
    spread_mean = spread.rolling(window=30).mean()
    spread_std = spread.rolling(window=30).std()
    zscore = (spread - spread_mean) / spread_std

    # Entry signals
    # Buy spread when z < -2 (spread oversold)
    # Sell spread when z > 2 (spread overbought)
    entries_long = zscore < -2
    exits_long = zscore > 0

    entries_short = zscore > 2
    exits_short = zscore < 0

    # Note: True pairs trading requires simultaneous long/short
    # This is simplified using the spread as a single instrument

    # Backtest on the spread
    pf = vbt.Portfolio.from_signals(
        spread,
        entries_long | entries_short,
        exits_long | exits_short,
        fees=0.001
    )

    print(f"\nStrategy Performance:")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"  Win Rate: {pf.trades.win_rate():.2%}")
    print(f"  Total Trades: {pf.trades.count()}")

    # Plot spread and z-score
    fig = spread.vbt.plot()
    fig.show()

    return pf, zscore


# ==============================================================================
# 3. HEDGE RATIO OPTIMIZATION
# ==============================================================================

def pairs_trading_with_hedge_ratio():
    """Use regression to find optimal hedge ratio"""

    from sklearn.linear_model import LinearRegression

    # Download data
    symbols = ['GLD', 'GDX']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate optimal hedge ratio using OLS
    X = price['GDX'].values.reshape(-1, 1)
    y = price['GLD'].values

    model = LinearRegression()
    model.fit(X, y)
    hedge_ratio = model.coef_[0]

    print("=" * 60)
    print("PAIRS TRADING WITH HEDGE RATIO")
    print("=" * 60)
    print(f"Optimal Hedge Ratio: {hedge_ratio:.4f}")
    print(f"Interpretation: 1 unit GLD = {hedge_ratio:.4f} units GDX")

    # Calculate spread using hedge ratio
    spread = price['GLD'] - hedge_ratio * price['GDX']

    # Z-score
    spread_mean = spread.rolling(30).mean()
    spread_std = spread.rolling(30).std()
    zscore = (spread - spread_mean) / spread_std

    # Signals
    entries_long = zscore < -2
    exits_long = zscore > 0

    entries_short = zscore > 2
    exits_short = zscore < 0

    # Backtest
    pf = vbt.Portfolio.from_signals(
        spread,
        entries_long | entries_short,
        exits_long | exits_short,
        fees=0.001
    )

    print(f"\nPerformance:")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"  Total Trades: {pf.trades.count()}")

    return pf, hedge_ratio


# ==============================================================================
# 4. ROLLING HEDGE RATIO
# ==============================================================================

def rolling_hedge_ratio_pairs():
    """Recalculate hedge ratio periodically"""

    from sklearn.linear_model import LinearRegression

    # Download data
    symbols = ['GLD', 'GDX']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate rolling hedge ratio
    window = 60  # 60-day rolling window
    hedge_ratios = []

    for i in range(window, len(price)):
        X = price['GDX'].iloc[i-window:i].values.reshape(-1, 1)
        y = price['GLD'].iloc[i-window:i].values

        model = LinearRegression()
        model.fit(X, y)
        hedge_ratios.append(model.coef_[0])

    # Pad with NaN for first window
    hedge_ratios = [np.nan] * window + hedge_ratios
    hedge_ratio_series = pd.Series(hedge_ratios, index=price.index)

    print("=" * 60)
    print("ROLLING HEDGE RATIO PAIRS TRADING")
    print("=" * 60)
    print(f"Window: {window} days")
    print(f"Mean Hedge Ratio: {hedge_ratio_series.mean():.4f}")
    print(f"Std Hedge Ratio: {hedge_ratio_series.std():.4f}")

    # Calculate spread with rolling hedge ratio
    spread = price['GLD'] - hedge_ratio_series * price['GDX']

    # Z-score
    spread_mean = spread.rolling(30).mean()
    spread_std = spread.rolling(30).std()
    zscore = (spread - spread_mean) / spread_std

    # Signals
    entries_long = zscore < -2
    exits_long = zscore > 0
    entries_short = zscore > 2
    exits_short = zscore < 0

    # Backtest
    pf = vbt.Portfolio.from_signals(
        spread,
        entries_long | entries_short,
        exits_long | exits_short,
        fees=0.001
    )

    print(f"\nPerformance:")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")

    return pf


# ==============================================================================
# 5. MULTIPLE PAIRS PORTFOLIO
# ==============================================================================

def multiple_pairs_portfolio():
    """Trade multiple pairs simultaneously"""

    # Define pairs to trade
    pairs = [
        ('GLD', 'GDX'),
        ('XLE', 'XOP'),
    ]

    all_results = []

    for asset1, asset2 in pairs:
        # Download data
        price = vbt.YFData.download([asset1, asset2], period='2y',
                                     missing_index='drop').get('Close')

        # Test cointegration
        score, pvalue, _ = coint(price[asset1], price[asset2])

        # Calculate spread
        spread = price[asset1] - price[asset2]

        # Z-score
        spread_mean = spread.rolling(30).mean()
        spread_std = spread.rolling(30).std()
        zscore = (spread - spread_mean) / spread_std

        # Signals
        entries = (zscore < -2) | (zscore > 2)
        exits = (zscore > -0.5) & (zscore < 0.5)

        # Backtest
        pf = vbt.Portfolio.from_signals(
            spread,
            entries,
            exits,
            fees=0.001
        )

        all_results.append({
            'pair': f"{asset1}/{asset2}",
            'pvalue': pvalue,
            'cointegrated': pvalue < 0.05,
            'total_return': pf.total_return(),
            'sharpe': pf.sharpe_ratio(),
            'max_dd': pf.max_drawdown(),
            'trades': pf.trades.count()
        })

    # Display results
    results_df = pd.DataFrame(all_results)

    print("=" * 60)
    print("MULTIPLE PAIRS PORTFOLIO")
    print("=" * 60)
    print(results_df.to_string(index=False))

    return results_df


# ==============================================================================
# 6. KALMAN FILTER PAIRS TRADING
# ==============================================================================

def kalman_filter_pairs():
    """Use Kalman filter for dynamic hedge ratio"""

    from pykalman import KalmanFilter

    # Download data
    symbols = ['GLD', 'GDX']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Set up Kalman Filter
    delta = 1e-3
    trans_cov = delta / (1 - delta) * np.eye(2)

    obs_mat = np.expand_dims(
        np.vstack([[price['GDX']], [np.ones(len(price['GDX']))]]).T,
        axis=1
    )

    kf = KalmanFilter(
        n_dim_obs=1,
        n_dim_state=2,
        initial_state_mean=[0, 0],
        initial_state_covariance=np.ones((2, 2)),
        transition_matrices=np.eye(2),
        observation_matrices=obs_mat,
        observation_covariance=2,
        transition_covariance=trans_cov
    )

    # Run filter
    state_means, _ = kf.filter(price['GLD'].values)

    # Extract hedge ratio and intercept
    hedge_ratio = pd.Series(state_means[:, 0], index=price.index)
    intercept = pd.Series(state_means[:, 1], index=price.index)

    print("=" * 60)
    print("KALMAN FILTER PAIRS TRADING")
    print("=" * 60)
    print(f"Dynamic Hedge Ratio")
    print(f"  Mean: {hedge_ratio.mean():.4f}")
    print(f"  Std: {hedge_ratio.std():.4f}")

    # Calculate spread
    spread = price['GLD'] - hedge_ratio * price['GDX'] - intercept

    # Z-score
    spread_mean = spread.rolling(30).mean()
    spread_std = spread.rolling(30).std()
    zscore = (spread - spread_mean) / spread_std

    # Signals
    entries = (zscore < -1.5) | (zscore > 1.5)
    exits = abs(zscore) < 0.5

    # Backtest
    pf = vbt.Portfolio.from_signals(
        spread,
        entries,
        exits,
        fees=0.001
    )

    print(f"\nPerformance:")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")

    return pf, hedge_ratio


# ==============================================================================
# 7. DISTANCE METHOD PAIRS TRADING
# ==============================================================================

def distance_method_pairs():
    """Normalized distance between prices"""

    # Download data
    symbols = ['GLD', 'GDX']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Normalize prices (z-score)
    gld_norm = (price['GLD'] - price['GLD'].rolling(60).mean()) / price['GLD'].rolling(60).std()
    gdx_norm = (price['GDX'] - price['GDX'].rolling(60).mean()) / price['GDX'].rolling(60).std()

    # Calculate distance
    distance = gld_norm - gdx_norm

    # Z-score of distance
    distance_mean = distance.rolling(30).mean()
    distance_std = distance.rolling(30).std()
    distance_zscore = (distance - distance_mean) / distance_std

    print("=" * 60)
    print("DISTANCE METHOD PAIRS TRADING")
    print("=" * 60)

    # Signals
    entries = (distance_zscore < -2) | (distance_zscore > 2)
    exits = abs(distance_zscore) < 0.5

    # Backtest on distance
    pf = vbt.Portfolio.from_signals(
        distance,
        entries,
        exits,
        fees=0.001
    )

    print(f"Performance:")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")

    return pf


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    print("\n" + "="*60)
    print("VECTORBT PAIRS TRADING EXAMPLES")
    print("="*60 + "\n")

    # 1. Find cointegrated pairs
    pairs_df, price_data = find_cointegrated_pairs()

    # 2. Basic pairs trading
    pf_basic, zscore = basic_pairs_trading()

    # 3. Hedge ratio optimization
    pf_hedge, hedge_ratio = pairs_trading_with_hedge_ratio()

    # 4. Rolling hedge ratio
    pf_rolling = rolling_hedge_ratio_pairs()

    # 5. Multiple pairs
    results_multi = multiple_pairs_portfolio()

    # 6. Kalman filter
    pf_kalman, kf_hedge = kalman_filter_pairs()

    # 7. Distance method
    pf_distance = distance_method_pairs()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
    print("\nNote: Pairs trading requires simultaneous long/short execution")
    print("Full implementation would use Portfolio.from_order_func()")
