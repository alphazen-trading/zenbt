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
def create_signal(size, close, atr, atr_multiplier: float, rr: float, tp_distance):
    limit_orders = np.zeros(((len(atr) - 15) * 2, 6))
    order_index = 0
    for i in range(15, len(atr)):
        # Place Long order
        entry_price = close[i - 1] - atr[i - 1] * atr_multiplier
        tp_price = entry_price + (close[i - 1] - entry_price) * tp_distance
        profit_amount = tp_price - entry_price
        sl_price = entry_price - profit_amount * rr
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
        entry_price = close[i - 1] + atr[i - 1] * atr_multiplier
        # sl_price = entry_price * (1 + sl_distance)
        tp_price = entry_price - (entry_price - close[i - 1]) * tp_distance
        profit_amount = entry_price - tp_price
        sl_price = entry_price + profit_amount * rr
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


def run_backtest(df, bt, size, atr_multiplier, rr, tp_distance):
    atr = talib.ATR(
        df["high"].to_numpy(),
        df["low"].to_numpy(),
        df["close"].to_numpy(),
        timeperiod=14,
    )

    limit_orders = create_signal(
        size, df["close"].to_numpy(), atr, atr_multiplier, rr, tp_distance
    )
    bt.prepare(limit_orders, atr)
    bt.backtest()
    pnl = 0
    unrealized_pnl = 0
    max_dd = 0
    commissions = 0
    wins = 0
    losses = 0
    for pos in bt.closed_positions:
        entry = pos.entry_price
        exit = pos.exit_price
        # pnl = (exit - entry) * 1000000
        pnl += pos.pnl
        max_dd += pos.max_dd
        commissions += pos.commission
        if pos.pnl < 0:
            losses += 1
        else:
            wins += 1
        # print()
        # print()
        # pos.print()
        # print("Entry: ", pos.entry_price)
        # print("Size: ", pos.size)
        # print("Commission: ", 0.02 / 100)
        # commission = float(pos.entry_price) * float(pos.size) * 0.02 / 100
        # print(commission)

    for pos in bt.active_positions:
        unrealized_pnl += pos.pnl
        max_dd += pos.max_dd
        commissions += pos.commission
        # print()
        # print("WE STILL HAVE ACTIVE POSITIONS")
        # pos.print()
    print(
        f"New backtest: atr_multiplier: {atr_multiplier}, rr: {rr}, tp_distance: {tp_distance}"
    )
    print("Total pnl: {}".format(pnl))
    print("Unrealized pnl: {}".format(unrealized_pnl))
    print(f"Total closed trades: {len(bt.closed_positions)}")
    print(f"Still open trades: {len(bt.active_positions)}")
    print(f"Max DD: {max_dd}")
    print(f"Commissions: {commissions}")
    print(f"Wins: {wins} -- Losses: {losses}")
    print(f"Win rate {np.round(wins / (wins + losses) * 100, 2)}%")
    print()
    return pnl
    # print(bt.closed_positions[0].print())


def download_data():
    df = OKXKlines().load_klines("BNB-USDT-SWAP", "1m", days_ago=90)


def main() -> int:
    # download_data()
    # df = pd.read_parquet("./data/kline_BTC-USDT-SWAP_1m.parquet")
    pnl = 0
    # for sym in ["DOGE", "BTC", "PEPE", "BNB"]:
    for sym in ["PEPE"]:
        df = pd.read_parquet(f"./data/kline_{sym}-USDT-SWAP_1m.parquet")

        df.sort_values(by=["date"], ascending=True, inplace=True)
        # df = df[-5000:]
        # print(df)

        df = df.resample("1min", on="d").agg(
            {
                "open": "first",
                "high": "max",
                "low": "min",
                "close": "last",
                "volume": "sum",
            }
        )
        # df.set_index("d", inplace=True)
        df.reset_index(inplace=True)
        df["d"] = pd.to_datetime(df["d"]).astype(int) / 10**6
        ohlc = df.to_numpy()
        COMMISSION = 0.02 / 100
        COMMISSION = 0
        bt = Backtest(ohlc, commission=COMMISSION)

        size = 10000 / df["close"][0]
        print(size)
        # size = 0.1
        for i in range(5, 6):
            print(sym)
            pnl += run_backtest(
                df, bt, size=size, atr_multiplier=i, rr=1 / 10, tp_distance=0.618
            )
    print(pnl)


# main()
