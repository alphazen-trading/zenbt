import polars as pl
from rich import print
from zenbt.rs import (
    Backtest,
    Action,
    BacktestParams,
    Side,
    cross_above,
    cross_below,
)
import talib
from sdk.base import DefaultAction, BaseStrategy

COMMISSION = 0.02 / 100
COMMISSION = 0
initial_capital = 100

bt_params = BacktestParams(
    commission_pct=COMMISSION,
    initial_capital=initial_capital,
    provide_active_position=False,
)


class ST(BaseStrategy):
    index = 0
    default_size = 1
    action = Action(orders={}, position=None, close_all_positions=False)

    def reset_action(self):
        self.action.reset()

    # def on_candle(self, index, state) -> Action:  # type: ignore
    def on_candle(self, state=None) -> Action:  # type: ignore
        self.index += 1
        # return self.on_candle_slow(index, state)
        # cross_below = self.data["cross_below"][index]
        # cross_above = self.data["cross_above"][index]
        if self.index == 100:
            order = self.create_market_order(
                self.index,
                client_order_id="Short",
                side=Side.Short,
                size=self.default_size,
            )
            # self.action.orders = {order.client_order_id: order}
        return self.action
        cross_above = False
        cross_below = False
        if index == 100:
            cross_below = True
        if index == 200:
            cross_above = True
        self.action.reset()

        # Check for bullish cross over
        if cross_above:
            order = self.create_market_order(
                index,
                client_order_id="Long",
                side=Side.Long,
                size=self.default_size,
            )
            self.action.orders = {order.client_order_id: order}
            self.action.close_all_positions = True

        # Check for bearish crossover
        if cross_below:
            order = self.create_market_order(
                index,
                client_order_id="Short",
                side=Side.Short,
                size=self.default_size,
            )
            self.action.orders = {order.client_order_id: order}
            self.action.close_all_positions = True

        return self.action


class ZBT:
    def __init__(self, df):
        self.df = df
        df = df.copy()
        df.reset_index(inplace=True)

        # df["time"] = df["Date"].astype(int) / 10**6
        df.rename(columns={"Date": "time"}, inplace=True)
        df["open"] = df["spy"].astype(float)
        df["high"] = df["spy"].astype(float)
        df["low"] = df["spy"].astype(float)
        df["close"] = df["spy"].astype(float)
        df["volume"] = df["spy"].astype(float)
        df.drop(["spy"], axis=1, inplace=True)

        df = pl.from_pandas(df)
        df = df.with_columns(pl.col("time").cast(pl.Datetime).cast(pl.Int64) // 10**3)
        fast_ma = talib.SMA(df["close"], timeperiod=10)
        slow_ma = talib.SMA(df["close"], timeperiod=50)
        # atr = talib.ATR(df["high"], df["low"], df["close"], timeperiod=14)
        df = df.with_columns(
            pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
            pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
        )
        st = ST(df)
        st.default_size = 2
        self.bt = Backtest(df, bt_params, st)

    def backtest(self):
        self.bt.backtest()
        return self.bt
