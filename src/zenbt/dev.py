import itertools
# from sdk.logger import print, logger

from zenbt.multi_backtest import multi_backtest
from binance import Client

from data.binance_klines import fetch_futures_data
import mplfinance as mpf
from zenbt.rs import Backtest, BacktestParams, create_limit_orders
import zenbt.rs as rs
import numpy as np
import pandas as pd
from rich import print
from tradingtoolbox.utils import resample

from strategy.atr import ATR_Strategy
from data.okx_klines import OKXKlines

pd.options.display.float_format = "{:.10f}".format


def run_backtest(df, ohlcs, size, st_params, bt_params):
    st = ATR_Strategy(df, size, st_params)
    limit_orders = st.limit_orders
    limit_orders = create_limit_orders(limit_orders)

    bt = Backtest(ohlcs, bt_params, limit_orders)
    bt.backtest()
    return bt

    # df["d"] = pd.to_datetime(df["d"], unit="ms")
    # df.set_index("d", inplace=True)
    # print(
    #     f"New backtest: atr_multiplier: {atr_multiplier}, rr: {rr}, tp_distance: {tp_distance}"
    # )
    print(bt.stats)
    # stats = Stats(bt, df)
    # print(stats.stats)
    return
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


COMMISSION = 0
COMMISSION = 0.02 / 100
initial_capital = 1000


def dev():
    bt_params = BacktestParams(
        commission_pct=COMMISSION, initial_capital=initial_capital
    )

    # fetch_futures_data(symbol="BTCUSDT", count=365)
    for sym in ["BTC"]:
        df, ohlcs = read_data(sym, 0, 1000, resample_tf="1min")

        # size = initial_capital / df["close"][0]
        size = 10000
        size = 0.001
        size = 0.01
        st_params = (2, 0.33, 2, True)
        st_params = (15, 1, 5, True)
        print("Running the backtest")
        bt = run_backtest(df, ohlcs, size, st_params, bt_params)

        a = bt.get_state()
        # print(a["floating_equity"])
        # print(a["equity"])
        # print(a["closed_positions"])
        print(a["stats"])
