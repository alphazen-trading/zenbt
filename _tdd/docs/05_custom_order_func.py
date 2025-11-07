"""
Custom Order Function - VectorBT Reference Implementation

Strategy Logic:
- Use Portfolio.from_order_func() for complex custom logic
- Implement state-based trading decisions
- Dynamic position sizing based on conditions
- Multiple orders per signal

Features Demonstrated:
- Custom order callbacks with from_order_func()
- State management across bars
- Dynamic position sizing
- Complex entry/exit logic

VectorBT Concepts Used:
- Portfolio.from_order_func() - Most powerful simulation method
- order_func_nb - Numba-compiled order callback
- Context object for accessing state
- pre_sim_func_nb for initialization
"""

import vectorbt as vbt
import pandas as pd
import numpy as np
from numba import njit


# ==============================================================================
# 1. BASIC CUSTOM ORDER FUNCTION
# ==============================================================================

@njit
def basic_order_func_nb(c):
    """
    Simple custom order function

    c: context object with:
        - c.i: current row index
        - c.close: close prices
        - c.col: current column
    """
    # Skip first 50 bars (need MA data)
    if c.i < 50:
        return vbt.portfolio.enums.order_nothing()

    # Access custom arrays (passed in)
    fast_ma = c.fast_ma[c.i, c.col]
    slow_ma = c.slow_ma[c.i, c.col]
    prev_fast = c.fast_ma[c.i - 1, c.col]
    prev_slow = c.slow_ma[c.i - 1, c.col]

    # Check for crossover
    cross_above = (prev_fast <= prev_slow) and (fast_ma > slow_ma)
    cross_below = (prev_fast >= prev_slow) and (fast_ma < slow_ma)

    # Generate orders
    if cross_above:
        # Buy signal - create long order
        return vbt.portfolio.enums.order_nb(
            size=1.0,  # 100% of available cash
            size_type=vbt.portfolio.enums.SizeType.TargetPercent,
            direction=vbt.portfolio.enums.Direction.LongOnly
        )

    if cross_below:
        # Sell signal - close position
        return vbt.portfolio.enums.order_nb(
            size=0.0,  # Target 0% position
            size_type=vbt.portfolio.enums.SizeType.TargetPercent,
            direction=vbt.portfolio.enums.Direction.LongOnly
        )

    # No action
    return vbt.portfolio.enums.order_nothing()


def basic_custom_order():
    """Run backtest with basic custom order function"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='2y').get('Close')

    # Calculate indicators
    fast_ma = vbt.MA.run(price, 10).ma.values
    slow_ma = vbt.MA.run(price, 50).ma.values

    # Broadcast to 2D arrays
    fast_ma_2d = fast_ma.reshape(-1, 1)
    slow_ma_2d = slow_ma.reshape(-1, 1)

    # Run backtest with custom function
    pf = vbt.Portfolio.from_order_func(
        price,
        basic_order_func_nb,
        fast_ma=fast_ma_2d,
        slow_ma=slow_ma_2d,
        fees=0.001
    )

    print("=" * 60)
    print("BASIC CUSTOM ORDER FUNCTION")
    print("=" * 60)
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")

    return pf


# ==============================================================================
# 2. DYNAMIC POSITION SIZING BASED ON VOLATILITY
# ==============================================================================

@njit
def volatility_sizing_nb(c):
    """Adjust position size based on volatility (ATR)"""

    if c.i < 50:
        return vbt.portfolio.enums.order_nothing()

    # Get indicators
    rsi = c.rsi[c.i, c.col]
    atr = c.atr[c.i, c.col]
    price = c.close[c.i, c.col]

    # Entry condition: RSI oversold
    if rsi < 30 and c.position_now == 0:
        # Calculate position size based on ATR
        # Risk 2% of capital per trade
        risk_pct = 0.02
        stop_distance = atr * 2  # 2x ATR stop

        # Position size = (Account × Risk%) / Stop Distance
        # Simplified: larger ATR = smaller position
        size_pct = min(1.0, risk_pct / (stop_distance / price))

        return vbt.portfolio.enums.order_nb(
            size=size_pct,
            size_type=vbt.portfolio.enums.SizeType.TargetPercent,
            direction=vbt.portfolio.enums.Direction.LongOnly
        )

    # Exit condition: RSI overbought
    if rsi > 70 and c.position_now > 0:
        return vbt.portfolio.enums.order_nb(
            size=0.0,
            size_type=vbt.portfolio.enums.SizeType.TargetPercent,
            direction=vbt.portfolio.enums.Direction.LongOnly
        )

    return vbt.portfolio.enums.order_nothing()


def volatility_based_sizing():
    """Position sizing based on ATR volatility"""

    # Download OHLC data
    data = vbt.YFData.download('BTC-USD', period='2y')
    close = data.get('Close')
    high = data.get('High')
    low = data.get('Low')

    # Calculate indicators
    rsi = vbt.RSI.run(close, 14).rsi.values.reshape(-1, 1)
    atr = vbt.ATR.run(high, low, close, 14).atr.values.reshape(-1, 1)

    # Run backtest
    pf = vbt.Portfolio.from_order_func(
        close,
        volatility_sizing_nb,
        rsi=rsi,
        atr=atr,
        fees=0.001
    )

    print("=" * 60)
    print("VOLATILITY-BASED POSITION SIZING")
    print("=" * 60)
    print("Risk per trade: 2% of capital")
    print("Stop distance: 2x ATR")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Win Rate: {pf.trades.win_rate():.2%}")

    return pf


# ==============================================================================
# 3. PYRAMIDING - ADD TO WINNERS
# ==============================================================================

@njit
def pyramiding_nb(c):
    """Add to winning positions (pyramiding)"""

    if c.i < 50:
        return vbt.portfolio.enums.order_nothing()

    fast_ma = c.fast_ma[c.i, c.col]
    slow_ma = c.slow_ma[c.i, c.col]
    price = c.close[c.i, c.col]

    # Initial entry
    if c.position_now == 0:
        if fast_ma > slow_ma:
            return vbt.portfolio.enums.order_nb(
                size=0.33,  # Start with 33%
                size_type=vbt.portfolio.enums.SizeType.TargetPercent,
                direction=vbt.portfolio.enums.Direction.LongOnly
            )

    # Add to position if profitable
    elif c.position_now > 0:
        # Get entry price (approximate from context)
        # In real implementation, you'd track this in state

        # If price moved up significantly and still bullish
        if fast_ma > slow_ma * 1.02:  # Fast MA 2% above slow
            # Add to position
            current_pct = c.position_now / c.cash_now
            if current_pct < 0.9:  # Don't exceed 90% invested
                return vbt.portfolio.enums.order_nb(
                    size=min(0.66, current_pct + 0.33),
                    size_type=vbt.portfolio.enums.SizeType.TargetPercent,
                    direction=vbt.portfolio.enums.Direction.LongOnly
                )

        # Exit if trend reverses
        if fast_ma < slow_ma:
            return vbt.portfolio.enums.order_nb(
                size=0.0,
                size_type=vbt.portfolio.enums.SizeType.TargetPercent,
                direction=vbt.portfolio.enums.Direction.LongOnly
            )

    return vbt.portfolio.enums.order_nothing()


def pyramiding_strategy():
    """Add to winning positions"""

    # Download data
    price = vbt.YFData.download('ETH-USD', period='2y').get('Close')

    # Calculate MAs
    fast_ma = vbt.MA.run(price, 10).ma.values.reshape(-1, 1)
    slow_ma = vbt.MA.run(price, 50).ma.values.reshape(-1, 1)

    # Run backtest
    pf = vbt.Portfolio.from_order_func(
        price,
        pyramiding_nb,
        fast_ma=fast_ma,
        slow_ma=slow_ma,
        fees=0.001
    )

    print("=" * 60)
    print("PYRAMIDING STRATEGY")
    print("=" * 60)
    print("Start: 33% position")
    print("Add: Up to 66% then 100%")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")

    return pf


# ==============================================================================
# 4. GRID TRADING STRATEGY
# ==============================================================================

@njit
def grid_trading_nb(c, grid_levels, grid_size):
    """
    Grid trading: buy at lower levels, sell at upper levels

    grid_levels: array of price levels
    grid_size: size per grid level
    """

    if c.i < 1:
        return vbt.portfolio.enums.order_nothing()

    price = c.close[c.i, c.col]

    # Find which grid level we're at
    for level_idx in range(len(grid_levels)):
        grid_price = grid_levels[level_idx]

        # Buy when price drops below grid level
        if price < grid_price and c.close[c.i-1, c.col] >= grid_price:
            return vbt.portfolio.enums.order_nb(
                size=grid_size,
                size_type=vbt.portfolio.enums.SizeType.Amount,
                direction=vbt.portfolio.enums.Direction.LongOnly
            )

        # Sell when price rises above grid level
        if price > grid_price and c.close[c.i-1, c.col] <= grid_price:
            if c.position_now >= grid_size:
                return vbt.portfolio.enums.order_nb(
                    size=-grid_size,
                    size_type=vbt.portfolio.enums.SizeType.Amount,
                    direction=vbt.portfolio.enums.Direction.LongOnly
                )

    return vbt.portfolio.enums.order_nothing()


def grid_trading_strategy():
    """Grid trading strategy"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='1y').get('Close')

    # Define grid (e.g., every $1000 for BTC)
    current_price = price.iloc[-100]  # Recent average
    grid_spacing = 1000
    num_levels = 10

    grid_levels = np.array([
        current_price - (i * grid_spacing)
        for i in range(-num_levels//2, num_levels//2)
    ])

    grid_size = 0.01  # Buy 0.01 BTC at each level

    # Run backtest
    pf = vbt.Portfolio.from_order_func(
        price,
        grid_trading_nb,
        grid_levels,
        grid_size,
        init_cash=100_000,
        fees=0.001
    )

    print("=" * 60)
    print("GRID TRADING STRATEGY")
    print("=" * 60)
    print(f"Grid spacing: ${grid_spacing}")
    print(f"Grid levels: {num_levels}")
    print(f"Size per level: {grid_size} units")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")
    print(f"Total Trades: {pf.trades.count()}")

    return pf


# ==============================================================================
# 5. MARTINGALE STRATEGY (DOUBLE DOWN ON LOSSES)
# ==============================================================================

@njit
def martingale_nb(c, base_size):
    """
    Martingale: double position size after losses
    WARNING: High risk strategy for demonstration only!
    """

    if c.i < 50:
        return vbt.portfolio.enums.order_nothing()

    rsi = c.rsi[c.i, c.col]

    # Entry: RSI oversold
    if rsi < 30 and c.position_now == 0:
        # Determine size based on previous trade
        # In full implementation, track last trade result
        # For now, use base size

        return vbt.portfolio.enums.order_nb(
            size=base_size,
            size_type=vbt.portfolio.enums.SizeType.Amount,
            direction=vbt.portfolio.enums.Direction.LongOnly
        )

    # Exit: RSI overbought
    if rsi > 70 and c.position_now > 0:
        return vbt.portfolio.enums.order_nb(
            size=0.0,
            size_type=vbt.portfolio.enums.SizeType.TargetPercent,
            direction=vbt.portfolio.enums.Direction.LongOnly
        )

    return vbt.portfolio.enums.order_nothing()


def martingale_strategy():
    """Martingale strategy (educational example)"""

    # Download data
    price = vbt.YFData.download('BTC-USD', period='1y').get('Close')

    # Calculate RSI
    rsi = vbt.RSI.run(price, 14).rsi.values.reshape(-1, 1)

    base_size = 0.1  # Start with 0.1 units

    # Run backtest
    pf = vbt.Portfolio.from_order_func(
        price,
        martingale_nb,
        base_size,
        rsi=rsi,
        init_cash=100_000,
        fees=0.001
    )

    print("=" * 60)
    print("MARTINGALE STRATEGY")
    print("=" * 60)
    print("WARNING: High-risk educational example")
    print(f"Base size: {base_size} units")
    print()
    print(f"Total Return: {pf.total_return():.2%}")
    print(f"Sharpe Ratio: {pf.sharpe_ratio():.2f}")
    print(f"Max Drawdown: {pf.max_drawdown():.2%}")

    return pf


# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

if __name__ == "__main__":
    print("\n" + "="*60)
    print("VECTORBT CUSTOM ORDER FUNCTION EXAMPLES")
    print("="*60 + "\n")

    # 1. Basic custom order function
    pf_basic = basic_custom_order()

    # 2. Volatility-based sizing
    pf_vol = volatility_based_sizing()

    # 3. Pyramiding
    pf_pyramid = pyramiding_strategy()

    # 4. Grid trading
    pf_grid = grid_trading_strategy()

    # 5. Martingale (educational)
    pf_martingale = martingale_strategy()

    print("\n" + "="*60)
    print("All examples completed!")
    print("="*60)
    print("\nNote: These strategies demonstrate VectorBT's from_order_func()")
    print("Migration to Rust requires implementing equivalent logic in")
    print("the Strategy.on_candle() method with state management.")
