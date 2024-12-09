import talib
import pandas as pd
import polars as pl
from zenbt.zbt import indicators

from tradingtoolbox.clickhouse import ClickhouseSync
from zenbt.data import get_sample_btc_data


def dev():
    df = get_sample_btc_data()

    window = 100
    ind_123 = indicators.indicator_123(
        window,
        df["high"].to_numpy(),
        df["low"].to_numpy(),
        talib.MAX(df["high"], window).to_numpy(),
        talib.MIN(df["low"].to_numpy(), window),
    )
    # print(ind_123.keys())
    # slow_ma = talib.SMA(df["close"], timeperiod=50)
    df = df.with_columns(
        pl.Series("point_1", ind_123["point_1"]),
        pl.Series("point_2", ind_123["point_2"]),
        pl.Series("point_3", ind_123["point_3"]),
        # pl.Series("order_values", ind_123["order_values"]),
        # pl.Series("sl_values", ind_123["sl_values"]),
    )

    df = df.to_pandas()
    df["time"] = pd.to_datetime(df["time"], unit="ms")
    print(df)

    ch = ClickhouseSync.create()
    ch.insert_df(df, "ohlc")
