from sdk.logger import print, logger

from zenbt.rs import BBO, OHLC, Backtest, Signal
import zenbt.rs as rs
import pandas as pd
from rich import print as rprint

from data.okx_klines import OKXKlines

# pd.options.display.float_format = "{:.10f}".format


def main() -> int:
    # print(dir(Backtest))
    # print(dir(OHLC))
    # ohlc = OHLC(1, 2, 3, 4, 5, 10)
    # ohlc.print

    # df = OKXKlines().load_klines("BTC-USDT-SWAP", "1s", days_ago=30)
    # df = OKXKlines().load_klines("PEPE-USDT-SWAP", "1s", days_ago=7)
    # df = OKXKlines().load_klines("MKR-USDT-SWAP", "1s", days_ago=7)
    # df = OKXKlines().load_klines("DOGE-USDT-SWAP", "1s", days_ago=7)

    # df = OKXKlines()._fetch_lines("DOGE-USDT-SWAP", "1s")

    # df["d"] = pd.to_datetime(df["date"], unit="ms")

    df = pd.read_parquet("./data/kline_PEPE-USDT-SWAP_1s.parquet")
    df.drop(columns=["open", "close", "volume"], inplace=True)
    df["d"] = df["d"] + pd.Timedelta(hours=4)
    df["high"] = df["high"] * 10000000
    df["low"] = df["low"] * 10000000
    # df.drop(columns=["d"], inplace=True)
    # bt = Backtest(df.to_numpy())
    df["TR"] = (df["high"] / df["low"] - 1) * 100
    df = df[df["TR"] > 0.3]
    signal = df["TR"] > 0.3
    rprint(signal)
    # rprint(df.tail(10))
    # rprint(len(df))
    # # bt.print
    # trades = pd.read_csv("./data/okx_trading_history.csv")
    # closes = trades[trades["PnL"] != 0]
    # pepe = closes[closes["Symbol"] == "PEPE-USDT-SWAP"]
    # # print(pepe.columns)
    # # rprint(len(pepe[["Time", "Action", "PnL", "Filled Price"]]))
    # print(len(pepe))
    # rprint(pepe["Time"].tail(100))

    return 0
