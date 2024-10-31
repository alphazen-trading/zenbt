from data.data import read_data, read_data_pl, download_okx_data
import time
import talib
from rich import print
import polars as pl
from sdk.base import BaseStrategy
from zenbt.dev import PySharedState
from zenbt.rs import (
    OrderType,
    Side,
    BacktestParams,
    Backtest,
    Action,
    Order,
    Position,
)

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
    def on_candle(self, state: PySharedState = None, **kwargs) -> Action:  # type: ignore
        atr = self.data["atr"][self.index]
        if state.active_position:
            print(state.active_position)
        else:
            order = self.create_limit_rder(
                self.index,
                client_order_id="Long",
                side=Side.Long,
                size=self.default_size,
            )

        # # Check for bullish cross over
        # if cross_above:
        #     order = self.create_market_order(
        #         self.index,
        #         client_order_id="Long",
        #         side=Side.Long,
        #         size=self.default_size,
        #     )
        #     self.action.orders = {order.client_order_id: order}
        #     self.action.close_all_positions = True

        # # Check for bearish crossover
        # if cross_below:
        #     order = self.create_market_order(
        #         self.index,
        #         client_order_id="Short",
        #         side=Side.Short,
        #         size=self.default_size,
        #     )
        #     self.action.orders = {order.client_order_id: order}
        #     self.action.close_all_positions = True

        return self.action


def dev():
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, 1000, resample_tf="1min", exchange="binance")
    # sym = "BTC"
    # df = read_data_pl(sym, 0, 200, resample_tf="1min", exchange="okx")

    # backtest_old(df)

    atr = talib.SMA(df["close"], timeperiod=10)
    df = df.with_columns(pl.Series("atr", atr))
    st = ATR(df, default_size=1)
    bt = Backtest(df, bt_params, st)

    start = time.time()
    bt.backtest()
    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(len(seen_pos))

    # stats = Stats(bt, df)
    # stats.print()
    return
