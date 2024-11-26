# ZenBt

A python financial backtesting engine written in rust.


## Moving Average Cross over example:


### Creating the class:


```python
from zenbt.sdk.base import BaseStrategy
from zenbt.zbt import (
    Side,
    Action,
)


class MaCross(BaseStrategy):
    default_size = 1_000_000

    def on_candle(self, state=None, **kwargs) -> Action:  # type: ignore
        # Get the Data at the index
        cross_below = self.data["cross_below"][self.index]
        cross_above = self.data["cross_above"][self.index]

        # Check for bullish cross over
        if cross_above:
            order = self.create_market_order(
                self.index,
                client_order_id="Long",
                side=Side.Long,
                size=self.default_size,
            )
            return Action(
                orders={order.client_order_id: order},
                close_all_positions=True,
            )

        # Check for bearish crossover
        if cross_below:
            order = self.create_market_order(
                self.index,
                client_order_id="Short",
                side=Side.Short,
                size=self.default_size,
            )
            return Action(
                orders={order.client_order_id: order},
                close_all_positions=True,
            )

        return self.action
```


### Running the backtest:

```python
from zenbt.data import get_sample_btc_data
from zenbt.strategies import MaCross
from zenbt.sdk import Stats
from zenbt.zbt import Backtest, cross_above, cross_below
import talib

# This will return a sample BTC dataset
df = get_sample_btc_data()


# We will use talib to generate the slow and fast MA data
fast_ma = talib.SMA(df["close"], timeperiod=10)
slow_ma = talib.SMA(df["close"], timeperiod=50)
df = df.with_columns(
    pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
    pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
)

# Initiate the MACross class we created above
ma_cross = MaCross(df, default_size=1)

# Initiate the backtest parameters
bt_params = BacktestParams(
    commission_pct=0.02 / 100,  # This is 2 bps
    initial_capital=100_000,
    provide_active_position=True,
)

# Create the Backtest engine
bt = Backtest(df, bt_params, ma_cross)

# Run the backtest
bt.backtest()

# Display the stats
stats = Stats(bt, df)
stats.print()
```


```python
               Trading Summary
┏━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Metric           ┃                   Value ┃
┡━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━┩
│ initial_capital  │              100,000.00 │
│ pnl              │                9,101.40 │
│ pnl_pct          │                    9.10 │
│ unrealized_pnl   │                   45.20 │
│ total_positions  │                  15,829 │
│ closed_positions │                  15,828 │
│ active_positions │                       1 │
│ commissions      │               32,792.58 │
│ wins             │                4,988.00 │
│ losses           │               10,840.00 │
│ win_rate         │                  31.51% │
│ trading_days     │                     380 │
│ start_date       │ 2023-08-21 18:33:00 UTC │
│ end_date         │ 2024-09-05 18:27:00 UTC │
│ max_drawdown     │                4,693.10 │
│ max_drawdown_pct │                    4.69 │
└──────────────────┴─────────────────────────┘
```

