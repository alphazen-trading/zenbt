import pandas as pd
import time
from zenbt.rs import OHLCs
from rich import print
import numpy as np
from strategy.atr import ATR_Strategy
from data.data import read_data
from zenbt.rs import Backtest, BacktestParams, create_limit_orders
from zenbt.rs import cross_above, cross_below, create_signals
import talib

pd.options.display.float_format = "{:.10f}".format


def run_backtest(df, ohlcs, size, st_params, bt_params):
    st = ATR_Strategy(df, size, st_params)
    limit_orders = st.limit_orders
    limit_orders = create_limit_orders(limit_orders)

    bt = Backtest(ohlcs, bt_params, limit_orders, {})
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
initial_capital = 10000000000000

bt_params = BacktestParams(commission_pct=COMMISSION, initial_capital=initial_capital)


def dev():
    # df, ohlcs = read_data("BTC", 0, 1000, resample_tf="1min")

    # # size = initial_capital / df["close"][0]
    # size = 10000
    # size = 0.001
    # size = 0.01
    # st_params = (2, 0.33, 2, True)
    # # st_params = (15, 1, 5, True)
    # print("Running the backtest")
    # bt = run_backtest(df, ohlcs, size, st_params, bt_params)

    # a = bt.get_state()
    # # print(a["floating_equity"])
    # # print(a["equity"])
    # # print(a["closed_positions"])
    # print(a["stats"])
    # print(a["closed_positions"])
    # return
    # df = pd.read_parquet("./data/btc.parquet")
    # # df = df[0:150]
    # ohlcs = OHLCs(df.to_numpy())
    # x = np.array([1.0, 2.0, 3.0])
    # mult(3.0, x)

    df, ohlcs = read_data("BTC", 0, 150, resample_tf="1min")
    close = df["close"].to_numpy()
    fast_ma = talib.SMA(close, timeperiod=10)
    slow_ma = talib.SMA(close, timeperiod=50)

    entries = np.full(len(close), False)
    cross_below(fast_ma, slow_ma, entries)

    exits = np.full(len(close), False)
    cross_above(slow_ma, fast_ma, exits)

    blank = np.full(len(close), False)

    # signals = create_signals(entries, exits, blank, blank)
    # signals = create_signals(entries)
    start = time.time()
    bt = Backtest(ohlcs, bt_params, {})
    bt.backtest_signals(entries, exits, blank, blank)

    elapsed_time_ms = (time.time() - start) * 1000
    print(f"Backtest took: {elapsed_time_ms:.2f} ms")
    # print(a["stats"])
    return

    start = time.time()
    # a = bt.get_state()
    a = bt.get_stats()
    elapsed_time_ms = (time.time() - start) * 1000
    print(f"Elapsed time to get stats: {elapsed_time_ms:.2f} ms")
    print(a["stats"])
    # print(a["active_positions"])
    # print(a["closed_positions"])
    # # print(entries)
    # print(fast_ma)
