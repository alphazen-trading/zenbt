import itertools
# from sdk.logger import print, logger

from zenbt.multi_backtest import multi_backtest
from binance import Client

from data.binance_klines import fetch_futures_data
import mplfinance as mpf
from zenbt.rs import BBO, OHLC, Backtest, Signals
import zenbt.rs as rs
import numpy as np
import pandas as pd
from rich import print
from tradingtoolbox.utils import resample

from strategy.atr import ATR_Strategy

from sdk.stats import Stats

from data.okx_klines import OKXKlines

pd.options.display.float_format = "{:.10f}".format


def plot(df):
    ema_values = talib.EMA(df["close"].to_numpy(), timeperiod=33)
    ema = mpf.make_addplot(
        ema_values,
        panel=0,
        color="lime",
        ylabel="ema",
    )

    subplots = [ema]
    mpf.plot(
        df,
        style="nightclouds",
        type="candle",
        volume=False,
        title="OHLC Chart",
        addplot=subplots,
        # alines=dict(alines=[line], colors=[color], linewidths=[2]),
    )


def run_backtest(df, bt, size, params):
    st = ATR_Strategy(df, size, params)
    limit_orders = st.limit_orders

    bt.prepare(limit_orders)
    bt.backtest()

    # df["d"] = pd.to_datetime(df["d"], unit="ms")
    # df.set_index("d", inplace=True)
    # print(
    #     f"New backtest: atr_multiplier: {atr_multiplier}, rr: {rr}, tp_distance: {tp_distance}"
    # )
    stats = Stats(bt, df)
    print(stats.stats)
    # stats.equity["realized_equity"].plot()
    stats.equity["unrealized_equity"].plot()
    # trades = stats.closed_positions
    # print(f"Number of trades: {len(trades)}")
    # rprint(trades)
    return stats.stats

    losers = trades[trades["pnl"] < 0]
    winners = trades[trades["pnl"] > 0]
    rprint(losers["pnl"].mean())
    rprint(winners["pnl"].mean())
    trades.to_csv("trades.csv")
    return stats.stats

    # rprint(stats.equity)
    # rprint(stats.equity.dtypes)
    # plt.show()
    # # plt.show()
    trades = stats.closed_positions
    trades.drop(
        columns=[
            "commission",
            "max_dd",
            "pnl",
            "side",
            "commission_pct",
            "exit_timestamp",
        ],
        inplace=True,
    )
    trades["d"] = trades["entry_price"] / trades["sl"]
    rprint(trades[["entry_price", "sl", "d"]])
    return pnl
    # print(bt.closed_positions[0].print())


def download_data():
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
    return df


def generate_bt_params():
    atr_multipler = np.arange(2, 20)
    rr = [0.3, 0.5, 1, 2]
    tp_distance = [0.33, 0.618, 0.786, 1, 1.5, 2]
    use_close = [True, False]

    # atr_multipler = np.arange(2, 3)
    # rr = [0.3, 0.5]
    # tp_distance = [0.33, 0.618]
    return list(itertools.product(atr_multipler, rr, tp_distance, use_close))


def _dev():
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
    rprint(df.tail(10))

    # df = read_data("PEPE")
    # df["d"] = pd.to_datetime(df["d"], unit="ms")
    # print(df)


COMMISSION = 0
COMMISSION = 0.02 / 100
initial_capital = 1000


def dev():
    # fetch_futures_data(symbol="BTCUSDT", count=365)
    for sym in ["BTC"]:
        print(f"Running for {sym}")
        df = read_data(sym, 0, -1, resample_tf="1min")
        print("Preparing the backtestg")
        bt = Backtest(
            df.to_numpy(), commission_pct=COMMISSION, initial_capital=initial_capital
        )
        # size = initial_capital / df["close"][0]
        size = 10000
        size = 0.001
        size = 0.01
        params = (2, 0.33, 2, True)
        params = (15, 1, 1, True)
        print("Running the backtest")
        run_backtest(df, bt, size, params)


dev()
