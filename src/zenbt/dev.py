import time
import talib
import polars as pl
from zenbt.rs import (
    cross_above,
    cross_below,
    BacktestParams,
    Backtest,
)
from zenbt.ma_cross import MaCross
from data.data import read_data_pl


COMMISSION = 0
COMMISSION = 0.02 / 100
initial_capital = 20000

bt_params = BacktestParams(
    commission_pct=COMMISSION,
    initial_capital=initial_capital,
    provide_active_position=True,
)


def dev():
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")
    # sym = "BTC"
    # df = read_data_pl(sym, 0, 200, resample_tf="1min", exchange="okx")

    fast_ma = talib.SMA(df["close"], timeperiod=10)
    slow_ma = talib.SMA(df["close"], timeperiod=50)
    # atr = talib.ATR(df["high"], df["low"], df["close"], timeperiod=14)
    df = df.with_columns(
        pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
        pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
    )
    ma_cross = MaCross(df, default_size=1)
    bt = Backtest(df, bt_params, ma_cross)

    start = time.time()
    bt.backtest()
    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(len(seen_pos))

    # stats = Stats(bt, df)
    # stats.print()
    return
    # print(len(bt.state.closed_positions))

    # bt = Backtest(df, bt_params, st)

    # start = time.time()

    # bt.backtest()
    # # print(bt.state.closed_positions)
    # # print(bt.state.active_positions)

    # print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # # print(df[950:971])
    # return
