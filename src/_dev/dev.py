import time
import talib
import polars as pl
from zenbt import bt as zbt
from zenbt.zbt import (
    cross_above,
    cross_below,
    BacktestParams,
    Backtest,
)
from zenbt.strategies import MaCross
from zenbt.data import read_data_pl


def dev():
    from zenbt.data import get_sample_btc_data
    from zenbt.strategies import MaCross
    from zenbt.sdk import Stats
    from zenbt.zbt import Backtest, cross_above, cross_below
    import talib

    df = get_sample_btc_data()
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")

    fast_ma = talib.SMA(df["close"], timeperiod=10)
    slow_ma = talib.SMA(df["close"], timeperiod=50)
    df = df.with_columns(
        pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
        pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
    )

    ma_cross = MaCross(df, default_size=1)
    bt_params = BacktestParams(
        commission_pct=0.02 / 100,  # This is 2 bps
        initial_capital=100_000,
        provide_active_position=True,
    )
    bt = Backtest(df, bt_params, ma_cross)

    start = time.time()
    bt.backtest()
    print(f"Backtest took: {(time.time() - start) * 1000:.2f} ms")

    stats = Stats(bt, df)
    stats.print()


def _dev():
    # sym = "BTC"
    # df = read_data_pl(sym, 0, 200, resample_tf="1min", exchange="okx")
    # atr = talib.ATR(df["high"], df["low"], df["close"], timeperiod=14)

    sym = "1000PEPE"
    df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")

    fast_ma = talib.SMA(df["close"], timeperiod=10)
    slow_ma = talib.SMA(df["close"], timeperiod=50)
    df = df.with_columns(
        pl.Series("cross_above", cross_above(fast_ma, slow_ma)),
        pl.Series("cross_below", cross_below(fast_ma, slow_ma)),
    )
    bt_params = BacktestParams(
        commission_pct=0.02 / 100,  # This is 2 bps
        initial_capital=100_000,
        provide_active_position=True,
    )
    ma_cross = MaCross(df, default_size=1)
    bt = Backtest(df, bt_params, ma_cross)

    start = time.time()
    bt.backtest()
    print(f"Backtest with rows: {(time.time() - start) * 1000:.2f} ms")
    # print(len(seen_pos))

    bt.get_stats()
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
