import time
from rich import print
import pandas as pd
from zenbt.sdk.stats import Stats
from zenbt.data.data import read_data_pl
from .bt import BT

from .zenbt import ZBT
from .vbt import VBT

# from .zenbt_signals import ZBT_signals

# from data.data import read_data_pl, read_data
import numpy as np


def time_execution(fn, *args, iterations=10):
    times = []
    bt = fn(*args)
    for _ in range(iterations):
        # print(f"Running {fn.__name__}...")
        start = time.time()
        fn(*args)
        end = time.time()

        times.append((end - start) * 1000)
    return np.mean(times), np.std(times), bt


def fetch_data(reload=False):
    import bt as _bt

    if reload:
        data = _bt.get("spy", start="2001-01-01")  # type: ignore
        data.to_csv("./data/sample_data.csv")
    else:
        data = pd.read_csv("./data/sample_data.csv")
        data["Date"] = pd.to_datetime(data["Date"])
        data.set_index("Date", inplace=True)
    return data


def bench_all():
    # ================================ #
    #            Preparation
    # ================================ #
    # DATA
    data = fetch_data(reload=False)

    # BT
    start = time.time()
    bt = BT(data.copy())
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"BT prepare time: {elapsed_time_ms:.2f} ms")

    # VBT -- need to run first to make sure numba is compiled
    start = time.time()
    vbt = VBT(data.copy())
    vbt.backtest()
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"VBT prepare time: {elapsed_time_ms:.2f} ms")

    # ZBT
    start = time.time()
    zbt = ZBT(data)
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"ZBT prepare time: {elapsed_time_ms:.2f} ms")

    # # ZBT
    # start = time.time()
    # zbt_signals = ZBT_signals(data)
    # elapsed_time_ms = (time.time() - start) * 1000
    # print(f"ZBT From Signals prepare time: {elapsed_time_ms:.2f} ms")

    # ================================ #
    #            Benchmarking
    # ================================ #
    avg_time, std_dev, bt = time_execution(bt.backtest)
    print(f"BT execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}")

    avg_time, std_dev, bt = time_execution(vbt.backtest)
    print(f"VBT execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}")

    avg_time, std_dev, bt = time_execution(zbt.backtest)
    print(f"ZBT execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}")

    # avg_time, std_dev, bt = time_execution(zbt_signals.backtest)
    # print(
    #     f"ZBT From Signals execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}"
    # )


def bench():
    iterations = 1
    sym = "1000PEPE"
    df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")
    df = df.to_pandas()
    df["spy"] = df["close"]
    df["Date"] = pd.to_datetime(df["time"], unit="ms")
    df.drop(["open", "high", "low", "close", "volume", "time"], axis=1, inplace=True)
    df.set_index("Date", inplace=True)
    data = df
    print(len(data))

    # # BT
    # start = time.time()
    # bt = BT(data.copy())
    # elapsed_time_ms = (time.time() - start) * 1000
    # print(f"BT prepare time: {elapsed_time_ms:.2f} ms")

    # VBT -- need to run first to make sure numba is compiled
    start = time.time()
    vbt = VBT(data.copy())
    vbt.backtest()
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"VBT prepare time: {elapsed_time_ms:.2f} ms")

    # ZBT
    start = time.time()
    zbt = ZBT(data)
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"ZBT prepare time: {elapsed_time_ms:.2f} ms")

    # # ZBT
    # start = time.time()
    # zbt_signals = ZBT_signals(data)
    # elapsed_time_ms = (time.time() - start) * 1000
    # print(f"ZBT From Signals prepare time: {elapsed_time_ms:.2f} ms")

    print("------------------------------------")
    # ================================ #
    #            Benchmarking
    # ================================ #
    # avg_time, std_dev, bt = time_execution(bt.backtest, iterations=iterations)
    # print(f"BT execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}")

    avg_time, std_dev, bt = time_execution(zbt.backtest, iterations=iterations)
    # stats = Stats(bt, data)
    # stats.print()
    print(f"ZBT execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}")

    # avg_time, std_dev, bt = time_execution(zbt_signals.backtest, iterations=iterations)
    # # stats = Stats(bt, data)
    # # stats.print()
    # print(
    #     f"ZBT From Signals execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}"
    # )

    # avg_time, std_dev, bt = time_execution(vbt.backtest, iterations=iterations)
    # print(f"VBT execution time: Mean time = {avg_time:.2f} ms, Std dev = {std_dev:.4f}")
    # vbt_trades = bt.positions.records_readable
    # # print(vbt_trades)
    # # print(bt.stats())
