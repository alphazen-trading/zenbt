from data.data import read_data, read_data_pl, download_okx_data
import time
import talib
from rich import print
import polars as pl
from sdk.base import BaseStrategy
from zenbt.rs import (
    OrderType,
    Side,
    Strategy,
    BacktestParams,
    PySharedState,
    Backtest,
    Action,
    Order,
    Position,
    cross_above,
    cross_below,
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


class ST(BaseStrategy):
    def on_candle(self, state=None, **kwargs) -> Action:  # type: ignore
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
            self.action.orders = {order.client_order_id: order}
            self.action.close_all_positions = True

        # Check for bearish crossover
        if cross_below:
            order = self.create_market_order(
                self.index,
                client_order_id="Short",
                side=Side.Short,
                size=self.default_size,
            )
            self.action.orders = {order.client_order_id: order}
            self.action.close_all_positions = True

        return self.action


def dev():
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")
    # sym = "BTC"
    # df = read_data_pl(sym, 0, 200, resample_tf="1min", exchange="okx")

    # backtest_old(df)

    fast_ma = talib.SMA(df["close"], timeperiod=10)
    slow_ma = talib.SMA(df["close"], timeperiod=50)
    # atr = talib.ATR(df["high"], df["low"], df["close"], timeperiod=14)
    df = df.with_columns(
        pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
        pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
    )
    st = ST(df, default_size=1)
    bt = Backtest(df, bt_params, st)

    start = time.time()
    bt.backtest()
    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(len(seen_pos))

    # stats = Stats(bt, df)
    # stats.print()
    return
    print(len(bt.state.closed_positions))

    bt = Backtest(df, bt_params, st)

    start = time.time()

    bt.backtest()
    # print(bt.state.closed_positions)
    # print(bt.state.active_positions)

    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(df[950:971])
    return
