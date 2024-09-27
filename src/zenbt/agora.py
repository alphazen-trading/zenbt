# type: ignore
# from sdk.logger import print, logger

import mplfinance as mpf
import talib
from zenbt.rs import BBO, OHLC, Backtest, Signals
import zenbt.rs as rs
import numpy as np
import pandas as pd
from rich import print as rprint
from numba import njit

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


@njit
def create_signal(size, open, signals, distance: float, sl_distance: float, tp_values):
    limit_orders = np.zeros(((len(signals)) * 2, 6))
    order_index = 0
    for i in range(1, len(signals)):
        tp_price = tp_values[i - 1]
        if np.isnan(tp_price):
            tp_price = 0

        # Place Long order
        entry_price = open[i - 1] * (1 - distance)
        sl_price = open[i - 1] * (1 - distance - sl_distance)
        limit_orders[order_index] = [
            i,
            1.0,
            entry_price,
            size,
            sl_price,
            tp_price,
        ]
        order_index += 1

        # Place Short order
        entry_price = open[i - 1] * (1 + distance)
        sl_price = open[i - 1] * (1 + distance + sl_distance)
        limit_orders[order_index] = [
            i,
            0.0,
            entry_price,
            size,
            sl_price,
            tp_price,
        ]
        order_index += 1

    return limit_orders


def analyze_trades():
    trades = pd.read_csv("./data/okx_trading_history.csv")
    print(trades["PnL"].sum())
    closes = trades[trades["PnL"] != 0]
    symbols = closes["Symbol"].unique()
    for sym in symbols:
        pepe = closes[closes["Symbol"] == sym]
        rprint(sym, pepe["PnL"].sum())


def download_data():
    df = OKXKlines().load_klines("PEPE-USDT-SWAP", "1m", days_ago=90)
    # df = OKXKlines().load_klines("DOGE-USDT-SWAP", "1s", days_ago=7)
    # df = OKXKlines().load_klines("BTC-USDT-SWAP", "1s", days_ago=30)
    # df = OKXKlines().load_klines("PEPE-USDT-SWAP", "1s", days_ago=7)
    # df = OKXKlines().load_klines("MKR-USDT-SWAP", "1s", days_ago=7)


def run_backtest(df, bt, ema_period, size, entry_distance, sl_distance):
    tp_prices = talib.EMA(df["close"].to_numpy(), timeperiod=ema_period)
    limit_orders = create_signal(
        size,
        df["open"].to_numpy(),
        df["signal"].to_numpy(),
        entry_distance / 100,
        sl_distance / 100,
        tp_prices,
    )
    bt.prepare(limit_orders, tp_prices)
    bt.backtest()
    pnl = 0
    unrealized_pnl = 0
    for pos in bt.closed_positions:
        entry = pos.entry_price
        exit = pos.exit_price
        # pnl = (exit - entry) * 1000000
        pnl += pos.pnl
        # pos.print()

    for pos in bt.active_positions:
        unrealized_pnl += pos.pnl
        # print()
        # print("WE STILL HAVE ACTIVE POSITIONS")
        # pos.print()
    print(
        f"New backtest: ema_period: {ema_period}, size: {size}, entry_distance: {entry_distance}, sl_distance: {sl_distance}"
    )
    print("Total pnl: {}".format(pnl))
    print("Unrealized pnl: {}".format(unrealized_pnl))
    print(f"Total closed trades: {len(bt.closed_positions)}")
    print(f"Still open trades: {len(bt.active_positions)}")
    print()


def main() -> int:
    download_data()
    df = pd.read_parquet("./data/kline_MKR-USDT-SWAP_1s.parquet")
    df.sort_values(by=["date"], ascending=True, inplace=True)
    one_min = df.resample("1min", on="d").agg(
        {"open": "first", "high": "max", "low": "min", "close": "last", "volume": "sum"}
    )
    df.set_index("d", inplace=True)
    ohlc = df.to_numpy()
    bt = Backtest(ohlc)

    # df.reset_index(inplace=True)
    df["TR"] = (df["high"] / df["low"] - 1) * 100
    df["signal"] = df["TR"] > 0.3

    # plot(df)
    # print(len(df[df["TR"] > 0.3]))

    run_backtest(df, bt, ema_period=55, size=1, entry_distance=0.3, sl_distance=100)
    run_backtest(df, bt, ema_period=21, size=1, entry_distance=1, sl_distance=100)
    run_backtest(df, bt, ema_period=33, size=1, entry_distance=1, sl_distance=100)
    run_backtest(df, bt, ema_period=55, size=1, entry_distance=1, sl_distance=100)
    run_backtest(df, bt, ema_period=21, size=1, entry_distance=2, sl_distance=100)
    run_backtest(df, bt, ema_period=33, size=1, entry_distance=2, sl_distance=100)
    run_backtest(df, bt, ema_period=55, size=1, entry_distance=2, sl_distance=100)
    # run_backtest(df, bt, ema_period=21, size=1, entry_distance=0.3, sl_distance=100)
    # run_backtest(df, bt, ema_period=33, size=1, entry_distance=0.5, sl_distance=1)

    print("Starting")
    print("Done")
    # bt.print()
    # rprint(df)

    # print()
    # rprint(df[df["TR"] > 0.3][["open", "close", "d"]])
    # rprint(df[long_signals == 1][["open", "close", "d"]])
    # rprint(entry_prices[84571])
    # rprint(long_signals[84571])

    # rprint(df[short_signals == 1])
    # bt = Backtest(df.to_numpy(), long_signals, short_signals)
    # bt.run()
    # bt.print
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


# main()
