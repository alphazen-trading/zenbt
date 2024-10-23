from data.data import read_data, read_data_pl, download_okx_data
import json
import time
import talib
from rich import print
import polars as pl
from zenbt.rs import (
    OrderType,
    Side,
    Strategy,
    BacktestParams,
    SharedState,
    Backtest,
    Action,
    Order,
    Position,
)

from typing import Optional

COMMISSION = 0
COMMISSION = 0.02 / 100
initial_capital = 2000

bt_params = BacktestParams(commission_pct=COMMISSION, initial_capital=initial_capital)


class BaseStrategy(Strategy):
    index: int
    state: SharedState

    def create_market_order(
        self,
        clientOrderId: str,
        side: Side,
        size: float,
        sl: Optional[float] = None,
        tp: Optional[float] = None,
    ) -> Order:
        return Order(
            index=self.index,
            clientOrderId=clientOrderId,
            order_type=OrderType.Market,
            side=side,
            size=size,
            price=None,
            sl=sl,
            tp=tp,
        )

    def _on_candle(self, index, state: SharedState) -> Action:  # type: ignore
        self.index = index
        self.state = state
        self.time = self.data["time"][index]
        self.open = self.data["open"][index]
        self.high = self.data["high"][index]
        self.low = self.data["low"][index]
        self.close = self.data["close"][index]

        return self.on_candle()

    def active_long_positions(self) -> list[Position]:
        positions = []
        for pos in self.state.active_positions:
            if pos.side == Side.Long:
                positions.append(pos)
        return positions

    def active_short_positions(self) -> list[Position]:
        positions = []
        for pos in self.state.active_positions:
            if pos.side == Side.Short:
                positions.append(pos)
        return positions

    def has_position(self) -> bool:
        return len(self.state.active_positions) > 0


DefaultAction = Action(desired_positions=[], desired_orders={})


class ST(BaseStrategy):
    def on_candle(self) -> Action:  # type: ignore
        index = self.index
        fast_ma = self.data["fast_ma"]
        slow_ma = self.data["slow_ma"]
        desired_orders = {}
        desired_positions = []

        if self.has_position():
            # long_positions = self.active_long_positions()
            # print(long_positions)
            # short_positions = self.active_short_positions()
            for pos in self.state.active_positions.values():
                print(pos)
            # print(short_positions)
            return DefaultAction

        # Check for bullish cross over
        if fast_ma[index - 1] < slow_ma[index - 1] and fast_ma[index] >= slow_ma[index]:
            order = self.create_market_order(
                clientOrderId="Long", side=Side.Long, size=1
            )
            desired_orders[order.clientOrderId] = order
            desired_positions = []

        # Check for bearish crossover
        if fast_ma[index - 1] > slow_ma[index - 1] and fast_ma[index] <= slow_ma[index]:
            order = self.create_market_order(
                clientOrderId="Short", side=Side.Short, size=1
            )
            desired_orders[order.clientOrderId] = order
            desired_positions = []

        return Action(
            desired_positions=desired_positions, desired_orders=desired_orders
        )


def dev():
    # download_okx_data(days_ago=2)
    # sym = "1000PEPE"
    # df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")
    sym = "BTC"
    df = read_data_pl(sym, 0, 100, resample_tf="1min", exchange="okx")

    fast_ma = talib.SMA(df["close"], timeperiod=10)
    slow_ma = talib.SMA(df["close"], timeperiod=50)
    atr = talib.ATR(df["high"], df["low"], df["close"], timeperiod=14)
    df = df.with_columns(
        pl.Series("fast_ma", fast_ma),
        pl.Series("slow_ma", slow_ma),
    )

    st = ST(df)
    bt = Backtest(df, bt_params, st)

    start = time.time()

    bt.backtest()

    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(df[950:971])
    return
