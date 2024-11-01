from data.data import read_data, read_data_pl, download_okx_data
import time
import pandas as pd
import talib
from rich import print
import polars as pl
from sdk.base import BaseStrategy
from zenbt.dev import PySharedState
import numpy as np
from zenbt.rs import (
    OrderType,
    Side,
    BacktestParams,
    Backtest,
    Action,
    Order,
    Position,
)

from tradingtoolbox.clickhouse import ClickhouseSync


from sdk.stats import Stats
from typing import Optional

COMMISSION = 0
COMMISSION = 0.02 / 100
initial_capital = 20000

bt_params = BacktestParams(
    commission_pct=COMMISSION,
    initial_capital=initial_capital,
    provide_active_position=True,
)


class ATR(BaseStrategy):
    last_pos_id = None

    def on_candle(self, state: PySharedState = None, **kwargs) -> Action:  # type: ignore
        rsi = self.get("rsi")

        if state.active_position is None:
            if rsi < 30:
                self.add_order(
                    self.create_market_order(
                        self.index,
                        client_order_id="Long",
                        side=Side.Long,
                        size=self.default_size,
                    )
                )
            elif rsi > 70:
                # print("Go short")
                open = self.get("open")
                close = self.get("close")
                low = self.get("low")

                self.add_order(
                    self.create_market_order(
                        self.index,
                        client_order_id="Short",
                        side=Side.Short,
                        size=self.default_size,
                        tp=low,
                    )
                )
        else:
            pos: Position = state.active_position
            atr_at_pos = self.get_at("atr", pos.entry_index)
            if pos.id != self.last_pos_id:
                self.last_pos_id = pos.id
                print(pos.id, pos.entry_index)
                print(atr_at_pos)
        #     print("We are in a position")

        return self.action


def add_data_to_ch(df):
    client = ClickhouseSync.create(
        host="localhost",
        port=8123,
        user="default",
        password="",
        database="default",
    )
    df = df.to_pandas()
    df["time"] = pd.to_datetime(df["time"], unit="ms")
    client.insert_df(df, "ohlc", drop=True)


def dev():
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, 100, resample_tf="1min", exchange="binance")
    # sym = "BTC"
    # df = read_data_pl(sym, 0, 200, resample_tf="1min", exchange="okx")

    # backtest_old(df)

    atr = talib.SMA(df["close"], timeperiod=10)
    rsi = talib.RSI(df["close"], timeperiod=14)
    df = df.with_columns(pl.Series("rsi", rsi), pl.Series("atr", atr))
    st = ATR(df, default_size=100_000)
    bt = Backtest(df, bt_params, st)

    start = time.time()
    bt.backtest()
    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(len(seen_pos))

    stats = Stats(bt, df)
    stats.print()
    return
