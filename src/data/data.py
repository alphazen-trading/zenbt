from zenbt.rs import OHLCs
import pandas as pd
from tradingtoolbox.utils import resample
from tradingtoolbox.exchanges.okx import OKXKlines


def download_okx_data():
    df = OKXKlines().load_klines("LTC-USDT-SWAP", "1m", days_ago=90)


def read_data(sym, start=0, end=-1, resample_tf="1min"):
    # df = pd.read_parquet(f"./data/kline_{sym}-USDT-SWAP_1m.parquet")
    # df.sort_values(by=["date"], ascending=True, inplace=True)
    df = pd.read_parquet(f"./data/binance-{sym}USDT-PERPETUAL-1m.parquet")
    df.drop(
        columns=[
            "taker_buy_volume",
            "quote_asset_volume",
            "close_time",
            "number_of_trades",
            "taker_buy_quote_asset_volume",
            "ignore",
        ],
        inplace=True,
    )
    df["volume"] = df["volume"].astype(float)
    df["open"] = df["open"].astype(float)
    df["high"] = df["high"].astype(float)
    df["low"] = df["low"].astype(float)
    df["close"] = df["close"].astype(float)
    df = df[start:end]
    # df = df[-5000:]
    # df = df[28000:30000]
    df.reset_index(inplace=True)
    if resample_tf != "1min":
        df = resample(df, tf=resample_tf, on="time")
        df.reset_index(inplace=True)
    df["time"] = pd.to_datetime(df["time"]).astype(int) / 10**6

    ohlcs = OHLCs(df.to_numpy())
    return df, ohlcs
