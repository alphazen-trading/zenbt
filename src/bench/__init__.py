import time
from rich import print
import pandas as pd
from .bt import BT
from .vbt import VBT
from .zenbt import ZBT

# from data.data import read_data_pl, read_data
import bt as _bt

# sym = "1000PEPE"
# df = read_data_pl(sym, 0, -1, resample_tf="1min", exchange="binance")
# df_pd = read_data(sym, 0, -1, resample_tf="1min", exchange="binance")


def fetch_data(reload=False):
    if reload:
        data = _bt.get("spy", start="2001-01-01")  # type: ignore
        data.to_csv("./data/sample_data.csv")
    else:
        data = pd.read_csv("./data/sample_data.csv")
        data["Date"] = pd.to_datetime(data["Date"])
        data.set_index("Date", inplace=True)
    return data


def bench():
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

    # ================================ #
    #            Benchmarking
    # ================================ #
    start = time.time()
    res, t = bt.backtest()
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"BT took: {elapsed_time_ms:.2f} ms")
    # print(t.positions)

    start = time.time()
    pf = vbt.backtest()
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"VBT took: {elapsed_time_ms:.2f} ms")
    # print(pf.stats())

    start = time.time()
    _zbt = zbt.backtest()
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"ZBT took: {elapsed_time_ms:.2f} ms")
    # print(_zbt.get_stats())
