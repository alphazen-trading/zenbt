from data.data import read_data, read_data_pl, download_okx_data
import time
import talib
from rich import print
import polars as pl
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

bt_params = BacktestParams(commission_pct=COMMISSION, initial_capital=initial_capital)


class BaseStrategy(Strategy):
    index: int
    state: PySharedState

    def create_market_order(
        self,
        index,
        client_order_id: str,
        side: Side,
        size: float,
        sl: Optional[float] = None,
        tp: Optional[float] = None,
    ) -> Order:
        return Order(
            index=index,
            client_order_id=client_order_id,
            order_type=OrderType.Market,
            side=side,
            size=size,
            price=None,
            sl=sl,
            tp=tp,
        )

    # def active_long_positions(self) -> list[Position]:
    #     positions = []
    #     for pos in self.state.active_positions:
    #         if pos.side == Side.Long:
    #             positions.append(pos)
    #     return positions

    # def active_short_positions(self) -> list[Position]:
    #     positions = []
    #     for pos in self.state.active_positions:
    #         if pos.side == Side.Short:
    #             positions.append(pos)
    #     return positions

    # def has_position(self) -> bool:
    #     return len(self.state.active_positions) > 0


DefaultAction = Action(positions={}, orders={}, position=None)
seen_pos = []


class ST(BaseStrategy):
    default_size = 1
    index = 0

    def on_candle(self) -> Action:  # type: ignore
        self.index += 1
        # return self.on_candle_slow(index, state)
        cross_above = self.data["cross_above"][self.index]
        cross_below = self.data["cross_below"][self.index]
        desired_orders = {}
        desired_positions = {}
        desired_position = None
        close_all_positions = False

        # if state.active_position:
        #     desired_position = state.active_position

        # Check for bullish cross over
        if cross_above:
            order = self.create_market_order(
                self.index,
                client_order_id="Long",
                side=Side.Long,
                size=self.default_size,
                # sl=self.close * 0.99,
                # tp=self.close * 1.01,
            )
            desired_orders[order.client_order_id] = order
            desired_positions = {}
            desired_position = None
            close_all_positions = True

        # Check for bearish crossover
        if cross_below:
            order = self.create_market_order(
                self.index,
                client_order_id="Short",
                side=Side.Short,
                size=self.default_size,
                # sl=self.close * 1.01,
                # tp=self.close * 0.99,
            )
            desired_orders[order.client_order_id] = order
            desired_positions = {}
            desired_position = None
            close_all_positions = True

        return Action(
            orders=desired_orders,
            position=desired_position,
            close_all_positions=close_all_positions,
        )

    def on_candle_slow(self, index, state) -> Action:  # type: ignore
        fast_ma = self.data["fast_ma"]
        slow_ma = self.data["slow_ma"]

        orders = {}
        positions = {}
        # Check for bullish cross over
        if fast_ma[index - 1] < slow_ma[index - 1] and fast_ma[index] >= slow_ma[index]:
            # print("Going LONG at: ", index)
            order = self.create_market_order(
                index,
                client_order_id="Long",
                side=Side.Long,
                size=1,
                # sl=self.close * 0.99,
                # tp=self.close * 1.01,
            )
            orders[order.client_order_id] = order
            positions = {}

        # Check for bearish crossover
        if fast_ma[index - 1] > slow_ma[index - 1] and fast_ma[index] <= slow_ma[index]:
            # print("Going SHORT at: ", index)
            # print("Going short")
            order = self.create_market_order(
                index,
                client_order_id="Short",
                side=Side.Short,
                size=1,
                # sl=self.close * 1.01,
                # tp=self.close * 0.99,
            )
            orders[order.client_order_id] = order
            positions = {}

        return Action(positions=positions, orders=orders)


def dev():
    # download_okx_data(days_ago=2)
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")
    # sym = "BTC"
    # df = read_data_pl(sym, 0, 200, resample_tf="1min", exchange="okx")

    # backtest_old(df)

    start = time.time()
    fast_ma = talib.SMA(df["close"], timeperiod=10)
    slow_ma = talib.SMA(df["close"], timeperiod=50)
    # atr = talib.ATR(df["high"], df["low"], df["close"], timeperiod=14)
    df = df.with_columns(
        pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
        pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
    )
    st = ST(df)
    st.default_size = 2
    bt = Backtest(df, bt_params, st)

    bt.backtest()
    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(len(seen_pos))
    # print(dir(bt.state))
    print(len(bt.state.closed_positions))
    stats = Stats(bt, df)
    stats.print()
    stats.equity.plot()

    return
    bt = Backtest(df, bt_params, st)

    start = time.time()

    bt.backtest()
    # print(bt.state.closed_positions)
    # print(bt.state.active_positions)

    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(df[950:971])
    return
