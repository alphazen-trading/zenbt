# from zenbt.rs import Signal, Foo, Bar

# sig = Signal()
# sig.method

# def dev():
#     bar = Bar()
#     bar.test()
#     print(bar.foo.test.inner)
#     bar.foo.test.mine = 50
#     print(bar.foo.test.mine)
#     # bar.foo.inner = 50
#     # print(bar.foo.inner)


# signal = Signal(index=0, order_type="BUY", side="LONG", signal_type="SMA_CROSS")


# import pandas as pd
# import time
# from sdk.plotter import plot_equity
# from sdk.trade_record import get_trades_df
# from zenbt.multi_backtest import multi_backtest
# from zenbt.rs import OHLCs
# from rich import print
# import numpy as np
# from strategy.atr import ATR_Strategy
from data.data import read_data, download_okx_data
# from zenbt.rs import Backtest, BacktestParams
# import talib

# pd.options.display.float_format = "{:.10f}".format


# def run_backtest(df, ohlcs, size, st_params, bt_params):
#     st = ATR_Strategy(df, size, st_params)

#     bt = Backtest(ohlcs, bt_params, st.limit_orders, {})
#     bt.backtest()
#     return bt


# COMMISSION = 0
# COMMISSION = 0.02 / 100
# initial_capital = 2000

# bt_params = BacktestParams(commission_pct=COMMISSION, initial_capital=initial_capital)


# # size = initial_capital / df["close"][0]
# size = {
#     "PEPE": 200,
#     "1000PEPE": 200,
#     "BTC": 0.01,
# }


# def analyze_simulations():
#     df = pd.read_parquet("./data/latest_simulation.parquet")
#     print(df.columns)
#     df.sort_values(by=["pnl"], ascending=False, inplace=True)
#     df.drop(
#         columns=[
#             "initial_capital",
#             "active_positions",
#             "closed_positions",
#             "max_drawdown_pct",
#         ],
#         inplace=True,
#     )
#     print(df.head(30))
#     print(df)


# # Function to calculate the average time over 1000 iterations
# def time_execution(fn, *args, iterations=1000):
#     times = []
#     for _ in range(iterations):
#         # print(f"Running {fn.__name__}...")
#         start = time.time()
#         fn(*args)
#         end = time.time()
#         times.append(end - start)
#     return np.mean(times), np.std(times)  # Return mean and standard deviation


# def bt_perf(param, df, ohlcvs):
#     st = ATR_Strategy(df, 1000, param)

#     # Measure time for ATR_Strategy
#     avg_time, std_dev = time_execution(ATR_Strategy, df, 10, param)
#     print(f"ATR_Strategy: Mean time = {avg_time:.4f} seconds, Std dev = {std_dev:.4f}")

#     # Measure time for Backtest creation
#     avg_time, std_dev = time_execution(Backtest, ohlcvs, bt_params, st.limit_orders)
#     print(
#         f"Backtest creation: Mean time = {avg_time:.4f} seconds, Std dev = {std_dev:.4f}"
#     )

#     # Measure time for Backtest execution
#     bt = Backtest(
#         ohlcvs, bt_params, st.limit_orders
#     )  # Create the backtest outside the loop
#     avg_time, std_dev = time_execution(bt.backtest)
#     print(
#         f"Backtest execution: Mean time = {avg_time:.4f} seconds, Std dev = {std_dev:.4f}"
#     )

#     # Measure time for stats generation
#     avg_time, std_dev = time_execution(bt.get_stats)
#     print(
#         f"Stats generation: Mean time = {avg_time:.4f} seconds, Std dev = {std_dev:.4f}"
#     )


# def bt_method(param, df, ohlcvs):
#     st = ATR_Strategy(df, 0.1, param)
#     bt = Backtest(ohlcvs, bt_params, st.limit_orders)
#     bt.backtest_with_cb()
#     return bt


# from zenbt.rs import LimitOrders, OrderType, Side
# from zenbt.rs import Strategy, BT, Bar, Foo, BBO, Signal

# sg = Signal()

# # bbo = BBO()
# # bbo.code
# # print(BB)


# class ST(Strategy):
#     def major(self):
#         print("IN major")
#         print(self.inner.open)
#         return 4


def dev():
    #     # download_okx_data(days_ago=2)
    sym = "PEPE"
    df, ohlcs = read_data(sym, 0, 100, resample_tf="1min", exchange="okx")


#     bt = Backtest(ohlcs, bt_params, LimitOrders(10))
#     print(bt.external)
#     print(bt.ohlc)
#     print(bt.external.curr)
#     # bt.backtest_with_cb()
#     # df, ohlcs = read_data(sym, 0, -1, resample_tf="1min", exchange="okx")
#     # bt = bt_method((1, 2, 10, 1), df, ohlcs)
#     # plot_equity(df, bt)

#     # res = multi_backtest(
#     #     df, ohlcs, ATR_Strategy.generate_bt_params(simple=False), bt_method
#     # )

#     # analyze_simulations()

#     # bt = bt_perf((15, 2, 2, 1), df, ohlcs)
#     # bt = bt_method((11, 2, 0.786, 1), df, ohlcs)
#     # print(bt.get_stats()["stats"])
#     return

#     # state = bt.get_state()

#     # plot_equity(df, bt)

#     # multi_backtest(df, bt, size, st.generate_bt_params(), run_backtest)

#     # return

#     # df, ohlcs = read_data("BTC", 0, -1, resample_tf="1min")
#     # close = df["close"].to_numpy()
#     # fast_ma = talib.SMA(close, timeperiod=10)
#     # slow_ma = talib.SMA(close, timeperiod=50)

#     # entries = np.full(len(close), False)
#     # cross_below(fast_ma, slow_ma, entries)

#     # exits = np.full(len(close), False)
#     # cross_above(fast_ma, slow_ma, exits)

#     # blank = np.full(len(close), False)

#     # # signals = create_signals(entries, exits, blank, blank)
#     # # signals = create_signals(entries)
#     # start = time.time()
#     # bt = Backtest(ohlcs, bt_params, {})
#     # bt.backtest_signals(entries, exits, exits, entries)

#     # elapsed_time_ms = (time.time() - start) * 1000
#     # print(f"Backtest took: {elapsed_time_ms:.2f} ms")
#     # start = time.time()
#     # a = bt.get_state()
#     # print(a["stats"])
#     # elapsed_time_ms = (time.time() - start) * 1000
#     # print(f"State took: {elapsed_time_ms:.2f} ms")
#     # return

#     # start = time.time()
#     # # a = bt.get_state()
#     # a = bt.get_stats()
#     # elapsed_time_ms = (time.time() - start) * 1000
#     # print(f"Elapsed time to get stats: {elapsed_time_ms:.2f} ms")
#     # print(a["stats"])
#     # # print(a["active_positions"])
#     # # print(a["closed_positions"])
#     # # # print(entries)
#     # # print(fast_ma)


# # dev()
