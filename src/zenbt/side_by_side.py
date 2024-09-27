# type: ignore
import pandas as pd
from rich import print

from data.data import read_data


def sbs():
    df = pd.read_parquet("./data/OKXtrades.parquet")
    df.set_index("datetime", inplace=True)
    print(df.columns)
    sym = df["instId"].unique()
    df = df[df["instId"] == "PEPE-USDT-SWAP"]
    df = df[0:90]
    df = df[["clOrdId", "side", "fillPnl", "fee", "instId", "fillSz", "fillMarkPx"]]
    df["date"] = df.index
    df["date"] = df["date"].dt.tz_localize("Asia/Bangkok")
    df["date"] = df["date"].dt.tz_convert("UTC")
    df.set_index("date", inplace=True)
    print(df.head(10))
    # for i in range(len(df)):
    #     id = df.iloc[i]["clOrdId"]
    #     # if "O00" in id:
    #     print(id)

    df = pd.read_parquet("./data/backtest_trades.parquet")
    df.drop(columns=["index", "exit_index", "commission"], inplace=True)
    df = df[::-1]
    print(df.head(10))

    df, ohlcs = read_data("PEPE", 0, -1, resample_tf="1min", exchange="okx")
    df["date"] = pd.to_datetime(df["date"], unit="ms")
    print(df)
