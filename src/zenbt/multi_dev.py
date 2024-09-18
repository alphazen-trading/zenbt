import itertools

from zenbt.multi_backtest import multi_backtest
from zenbt.rs import Backtest
import numpy as np
import pandas as pd
from rich import print


def generate_bt_params():
    atr_multipler = np.arange(2, 20)
    rr = [0.3, 0.5, 1, 2]
    tp_distance = [0.33, 0.618, 0.786, 1, 1.5, 2]
    use_close = [True, False]

    # atr_multipler = np.arange(2, 3)
    # rr = [0.3, 0.5]
    # tp_distance = [0.33, 0.618]
    return list(itertools.product(atr_multipler, rr, tp_distance, use_close))


def dev():
    for sym in ["DOGE"]:
        df = read_data(sym, 0, -1, resample_tf="5min")
        print(len(df))
        bt = Backtest(df.to_numpy(), commission_pct=COMMISSION, initial_capital=10000)
        size = initial_capital / df["close"][0]
        multi_backtest(df, bt, size, generate_bt_params(), run_backtest)


def _dev():
    # fetch_futures_data(symbol="BNBUSDT", count=365)
    # download_data()
    df = pd.read_parquet("./data/simulation_result_20240906_185750.parquet")

    df["pnl"] = df["pnl"] + df["unrealized_pnl"]
    df.sort_values(by=["pnl"], inplace=True)
    # df = df["pnl"]
    print(df.tail(10))

    # df = read_data("PEPE")
    # df["d"] = pd.to_datetime(df["d"], unit="ms")
    # print(df)
