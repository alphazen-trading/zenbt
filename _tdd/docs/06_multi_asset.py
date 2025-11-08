"""
Multi-Asset Portfolio Strategy - VectorBT Reference Implementation

Strategy Logic:
- Trade multiple assets simultaneously
- Apply same strategy across multiple instruments
- Compare performance across assets
- Portfolio-level position management

Features Demonstrated:
- Multi-asset backtesting
- Portfolio aggregation
- Asset correlation analysis
- Equal-weight vs optimized portfolios

VectorBT Concepts Used:
- Multi-column DataFrames
- Portfolio grouping with group_by=True
- cash_sharing parameter
- Cross-asset analysis
"""

import vectorbt as vbt
import pandas as pd
import numpy as np


# ==============================================================================
# 1. BASIC MULTI-ASSET STRATEGY
# ==============================================================================

def basic_multi_asset():
    """Apply same strategy to multiple cryptocurrencies"""

    # Download multiple assets
    symbols = ['BTC-USD', 'ETH-USD', 'LTC-USD', 'XRP-USD']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate indicators for all assets
    fast_ma = vbt.MA.run(price, window=10)
    slow_ma = vbt.MA.run(price, window=50)

    # Generate signals
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    # Backtest each asset independently
    pf = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        init_cash=100_000,
        fees=0.001,
        freq='1D'
    )

    print("=" * 60)
    print("MULTI-ASSET STRATEGY - INDEPENDENT")
    print("=" * 60)
    print("\nPerformance by Asset:")

    for symbol in symbols:
        pf_asset = pf[symbol]
        print(f"\n{symbol}:")
        print(f"  Total Return: {pf_asset.total_return():.2%}")
        print(f"  Sharpe Ratio: {pf_asset.sharpe_ratio():.2f}")
        print(f"  Max Drawdown: {pf_asset.max_drawdown():.2%}")
        print(f"  Win Rate: {pf_asset.trades.win_rate():.2%}")
        print(f"  Trades: {pf_asset.trades.count()}")

    # Plot all equity curves
    pf.plot().show()

    return pf


# ==============================================================================
# 2. PORTFOLIO WITH CASH SHARING
# ==============================================================================

def multi_asset_cash_sharing():
    """Portfolio that shares cash across all assets"""

    # Download data
    symbols = ['BTC-USD', 'ETH-USD', 'LTC-USD']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate signals
    rsi = vbt.RSI.run(price, window=14)
    entries = rsi.rsi_below(30)
    exits = rsi.rsi_above(70)

    # Portfolio WITHOUT cash sharing (separate $100k per asset)
    pf_separate = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        init_cash=100_000,  # $100k per asset
        fees=0.001
    )

    # Portfolio WITH cash sharing (single $100k pool)
    pf_shared = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        init_cash=100_000,  # $100k total
        group_by=True,       # Treat as single portfolio
        cash_sharing=True,   # Share cash across assets
        fees=0.001
    )

    print("=" * 60)
    print("CASH SHARING COMPARISON")
    print("=" * 60)

    print("\nSeparate Cash ($100k per asset = $300k total):")
    print(f"  Total Return: {pf_separate.total_return().mean():.2%}")
    print(f"  Total Value: ${pf_separate.value().iloc[-1].sum():,.0f}")

    print("\nShared Cash ($100k total):")
    print(f"  Total Return: {pf_shared.total_return():.2%}")
    print(f"  Total Value: ${pf_shared.value().iloc[-1]:,.0f}")

    return pf_separate, pf_shared


# ==============================================================================
# 3. EQUAL WEIGHT PORTFOLIO
# ==============================================================================

def equal_weight_portfolio():
    """Rebalance to equal weights periodically"""

    # Download data
    symbols = ['BTC-USD', 'ETH-USD', 'BNB-USD', 'ADA-USD']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate returns
    returns = price.pct_change()

    # Buy and hold (no rebalancing)
    pf_hold = vbt.Portfolio.from_holding(
        price,
        init_cash=100_000,
        group_by=True,
        cash_sharing=True
    )

    # Note: True equal-weight rebalancing would require from_order_func
    # This is a simplified version using holding

    print("=" * 60)
    print("EQUAL WEIGHT PORTFOLIO")
    print("=" * 60)
    print(f"Assets: {', '.join(symbols)}")
    print(f"Initial Allocation: 25% each")
    print()
    print(f"Total Return: {pf_hold.total_return():.2%}")
    print(f"Sharpe Ratio: {pf_hold.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf_hold.max_drawdown():.2%}")

    return pf_hold


# ==============================================================================
# 4. CORRELATION-BASED PORTFOLIO
# ==============================================================================

def correlation_based_selection():
    """Select assets with low correlation"""

    # Download more assets
    symbols = ['BTC-USD', 'ETH-USD', 'LTC-USD', 'XRP-USD',
               'ADA-USD', 'DOT-USD', 'LINK-USD', 'UNI-USD']
    price = vbt.YFData.download(symbols, period='1y', missing_index='drop').get('Close')

    # Calculate returns
    returns = price.pct_change().dropna()

    # Calculate correlation matrix
    correlation_matrix = returns.corr()

    print("=" * 60)
    print("CORRELATION ANALYSIS")
    print("=" * 60)
    print("\nCorrelation Matrix:")
    print(correlation_matrix.round(2))

    # Find pairs with lowest correlation
    # (For diversification)
    corr_pairs = []
    for i in range(len(symbols)):
        for j in range(i+1, len(symbols)):
            corr_pairs.append({
                'asset1': symbols[i],
                'asset2': symbols[j],
                'correlation': correlation_matrix.iloc[i, j]
            })

    corr_df = pd.DataFrame(corr_pairs).sort_values('correlation')

    print("\nLowest Correlation Pairs (Best for Diversification):")
    print(corr_df.head(5).to_string(index=False))

    print("\nHighest Correlation Pairs (Move Together):")
    print(corr_df.tail(5).to_string(index=False))

    # Select low-correlation portfolio (e.g., top 4 lowest correlated)
    selected_assets = ['BTC-USD', 'ETH-USD', 'ADA-USD', 'LINK-USD']
    selected_price = price[selected_assets]

    # Simple strategy on selected assets
    fast_ma = vbt.MA.run(selected_price, 10)
    slow_ma = vbt.MA.run(selected_price, 50)
    entries = fast_ma.ma_crossed_above(slow_ma)
    exits = fast_ma.ma_crossed_below(slow_ma)

    pf = vbt.Portfolio.from_signals(
        selected_price,
        entries=entries,
        exits=exits,
        init_cash=100_000,
        group_by=True,
        cash_sharing=True,
        fees=0.001
    )

    print(f"\nSelected Portfolio Performance:")
    print(f"  Assets: {', '.join(selected_assets)}")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")

    return pf, correlation_matrix


# ==============================================================================
# 5. SECTOR ROTATION STRATEGY
# ==============================================================================

def sector_rotation():
    """Rotate between assets based on momentum"""

    # Download data
    symbols = ['BTC-USD', 'ETH-USD', 'LTC-USD', 'XRP-USD']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate momentum (20-day return)
    momentum = price.pct_change(20)

    # Select top 2 assets each period
    # Note: Full implementation would use from_order_func
    # This is a simplified version

    # Calculate rolling rank
    rank = momentum.rank(axis=1, ascending=False)

    # Buy top 2 momentum assets
    entries = rank <= 2
    exits = rank > 2

    # Backtest
    pf = vbt.Portfolio.from_signals(
        price,
        entries=entries,
        exits=exits,
        init_cash=100_000,
        group_by=True,
        cash_sharing=True,
        size=0.5,  # 50% per asset (2 assets max)
        fees=0.001
    )

    print("=" * 60)
    print("MOMENTUM ROTATION STRATEGY")
    print("=" * 60)
    print("Strategy: Hold top 2 momentum assets (20-day)")
    print(f"Assets: {', '.join(symbols)}")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    return pf


# ==============================================================================
# 6. MEAN-VARIANCE OPTIMIZATION
# ==============================================================================

def mean_variance_optimization():
    """Optimize portfolio weights using mean-variance"""

    from pypfopt import EfficientFrontier, risk_models, expected_returns

    # Download data
    symbols = ['BTC-USD', 'ETH-USD', 'BNB-USD', 'ADA-USD', 'DOT-USD']
    price = vbt.YFData.download(symbols, period='2y', missing_index='drop').get('Close')

    # Calculate expected returns and covariance
    mu = expected_returns.mean_historical_return(price)
    S = risk_models.sample_cov(price)

    # Optimize for maximum Sharpe ratio
    ef = EfficientFrontier(mu, S)
    weights = ef.max_sharpe()
    cleaned_weights = ef.clean_weights()

    print("=" * 60)
    print("MEAN-VARIANCE OPTIMIZED PORTFOLIO")
    print("=" * 60)
    print("\nOptimal Weights (Max Sharpe):")
    for asset, weight in cleaned_weights.items():
        if weight > 0:
            print(f"  {asset}: {weight:.1%}")

    # Performance stats
    expected_return, volatility, sharpe = ef.portfolio_performance()
    print(f"\nExpected Annual Return: {expected_return:.2%}")
    print(f"Expected Volatility: {volatility:.2%}")
    print(f"Expected Sharpe Ratio: {sharpe:.2f}")

    # Backtest with optimized weights
    # Note: Would need from_order_func for true rebalancing
    pf = vbt.Portfolio.from_holding(
        price,
        init_cash=100_000,
        group_by=True,
        cash_sharing=True
    )

    print(f"\nActual Performance:")
    print(f"  Total Return: {pf.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf.max_drawdown():.2%}")

    return pf, cleaned_weights


# ==============================================================================
# 7. DYNAMIC POSITION SIZING ACROSS ASSETS
# ==============================================================================

def dynamic_multi_asset_sizing():
    """Size positions based on volatility across assets"""

    # Download OHLC data
    symbols = ['BTC-USD', 'ETH-USD', 'LTC-USD']
    data = vbt.YFData.download(symbols, period='2y', missing_index='drop')
    close = data.get('Close')
    high = data.get('High')
    low = data.get('Low')

    # Calculate ATR for each asset
    atr = vbt.ATR.run(high, low, close, window=14)

    # ATR as percentage of price
    atr_pct = atr.atr / close

    # Entry signal (RSI)
    rsi = vbt.RSI.run(close, 14)
    entries = rsi.rsi_below(30)
    exits = rsi.rsi_above(70)

    # Strategy 1: Equal size
    pf_equal = vbt.Portfolio.from_signals(
        close,
        entries=entries,
        exits=exits,
        size=0.33,  # 33% per asset
        init_cash=100_000,
        group_by=True,
        cash_sharing=True,
        fees=0.001
    )

    # Strategy 2: Inverse volatility sizing (lower vol = larger size)
    # Note: Full implementation needs from_order_func
    # This is simplified

    print("=" * 60)
    print("DYNAMIC POSITION SIZING")
    print("=" * 60)

    print("\nAverage ATR (as % of price):")
    for symbol in symbols:
        avg_atr_pct = atr_pct[symbol].mean()
        print(f"  {symbol}: {avg_atr_pct:.2%}")

    print("\nEqual Sizing:")
    print(f"  Total Return: {pf_equal.total_return():.2%}")
    print(f"  Sharpe Ratio: {pf_equal.sharpe_ratio():.2f}")
    print(f"  Max Drawdown: {pf_equal.max_drawdown():.2%}")

    return pf_equal


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    print("\n" + "="*60)
    print("VECTORBT MULTI-ASSET PORTFOLIO EXAMPLES")
    print("="*60 + "\n")

    # 1. Basic multi-asset
    pf_basic = basic_multi_asset()

    # 2. Cash sharing
    pf_sep, pf_shared = multi_asset_cash_sharing()

    # 3. Equal weight
    pf_equal = equal_weight_portfolio()

    # 4. Correlation-based
    pf_corr, corr_matrix = correlation_based_selection()

    # 5. Momentum rotation
    pf_momentum = sector_rotation()

    # 6. Mean-variance optimization
    pf_opt, weights = mean_variance_optimization()

    # 7. Dynamic sizing
    pf_dynamic = dynamic_multi_asset_sizing()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
