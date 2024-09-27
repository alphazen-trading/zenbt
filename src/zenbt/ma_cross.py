# import pandas as pd
# from rich import print as rprint
# import talib
# import numpy as np
# from numba import njit
# from zenbt.rs import BBO, OHLC, Backtest, BacktestParams

# from zenbt.rs import cross_above, cross_below, Order


# def convert_signals_to_orders(long_entries, long_exits, short_entries, short_exits):
#     limit_orders = np.zeros(((len(long_entries)) * 2, 6))
#     for i in range(len(long_entries)):
#         if long_entries[i]:
#             order = Order(i, 1.0, 0.0, 0.0, 0.0, 0.0)
#             print("We got a new order")
#             break
#     order.print
#     return limit_orders


def ma_cross():
    print("In ma cross")
    # df = pd.read_parquet("./data/btc_small.parquet")
    # close = df["close"].to_numpy()
    # fast_ma = talib.EMA(close, timeperiod=10)
    # slow_ma = talib.EMA(close, timeperiod=50)

    # entries = cross_above(fast_ma, slow_ma)
    # exits = cross_below(fast_ma, slow_ma)
    # convert_signals_to_orders(entries, exits, exits, entries)
    # # print(entries)
    # # print(exits)
    # # print(df)
    # params = BacktestParams(commission_pct=0, initial_capital=100)

    # bt = Backtest(df.to_numpy(), params)
    # a = bt.get_data_as_dict()
    # rprint(a["active_positions"])
