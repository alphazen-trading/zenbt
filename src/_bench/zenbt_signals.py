import polars as pl
import numpy as np
from rich import print
from zenbt.rs import (
    BacktestParams,
    OHLCs,
    BacktestOld,
    LimitOrders,
    cross_above,
    cross_below,
)
import talib

COMMISSION = 0.02 / 100
COMMISSION = 0
initial_capital = 100

bt_params = BacktestParams(
    commission_pct=COMMISSION,
    initial_capital=initial_capital,
    provide_active_position=False,
)


class ZBT_signals:
    def __init__(self, df):
        self.df = df
        df.reset_index(inplace=True)
        df["Date"] = df["Date"].astype(int) / 10**6
        df["open"] = df["spy"].astype(float)
        df["high"] = df["spy"].astype(float)
        df["low"] = df["spy"].astype(float)
        df["close"] = df["spy"].astype(float)
        df["volume"] = df["spy"].astype(float)
        df.drop(["spy"], axis=1, inplace=True)

        ohlcs = OHLCs(df.to_numpy())

        df = pl.from_pandas(df)
        fast_ma = talib.SMA(df["close"], timeperiod=10)
        slow_ma = talib.SMA(df["close"], timeperiod=50)
        df = df.with_columns(
            pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
            pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
        )

        self.exits = df["cross_above"].to_numpy()
        self.entries = df["cross_above"].to_numpy()

        self.blank = np.full(len(df["close"]), False)

        self.bt = BacktestOld(ohlcs, bt_params, LimitOrders(10))

    def backtest(self):
        self.bt.backtest_signals(self.entries, self.exits, self.blank, self.blank)
        return self.bt
