from data.data import read_data, read_data_pl, download_okx_data
import time
import talib
import polars as pl
from zenbt.rs import (
    Strategy,
    BacktestParams,
    SharedState,
    Backtest,
    Decision,
    Action,
)


COMMISSION = 0
COMMISSION = 0.02 / 100
initial_capital = 2000

bt_params = BacktestParams(commission_pct=COMMISSION, initial_capital=initial_capital)


class ST(Strategy):
    def on_candle(self, index, state: SharedState, **kwargs):  # type: ignore
        time = self.data["time"][index]
        open = self.data["open"][index]
        close = self.data["close"][index]
        fast_ma = self.data["fast_ma"]
        slow_ma = self.data["slow_ma"]
        # print(len(self.state.equity))

        # Check for bullish crossover
        if fast_ma[index - 1] < slow_ma[index - 1] and fast_ma[index] >= slow_ma[index]:
            print("Bullish crossover detected")

        # Check for bearish crossover
        if fast_ma[index - 1] > slow_ma[index - 1] and fast_ma[index] <= slow_ma[index]:
            print("Bearish crossover detected")
        # print("-----")
        # print(len(state.equity))
        # print(state.equity[-1])

        res = Action(2)
        return res


def dev():
    # download_okx_data(days_ago=2)
    # sym = "1000PEPE"
    # df = read_data_pl(sym, 0, 1000, resample_tf="1min", exchange="binance")
    sym = "PEPE"
    df = read_data_pl(sym, 0, 1000, resample_tf="1min", exchange="okx")

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
    return
