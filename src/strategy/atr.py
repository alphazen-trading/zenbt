import talib
import itertools
from numba import njit
import numpy as np
import time

from zenbt.rs import create_limit_orders
from zenbt.rs import LimitOrders, OrderType, Side


@njit(cache=True)
def old_create_signal(
    size,
    close,
    high,
    low,
    atr,
    atr_multiplier: float,
    rr: float,
    tp_distance,
    use_close=True,
):
    limit_orders = np.zeros(((len(atr) - 15) * 2, 6))
    order_index = 0
    for i in range(15, len(atr)):
        # Place Long order
        price = close[i - 1]
        if not use_close:
            price = low[i - 1]
        entry_price = price - atr[i - 1] * atr_multiplier
        tp_price = entry_price + (price - entry_price) * tp_distance
        profit_amount = tp_price - entry_price
        sl_price = entry_price - profit_amount * rr
        # tp_price = 999999999999999999
        # sl_price = 0

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
        price = close[i - 1]
        if not use_close:
            price = high[i - 1]
        entry_price = price + atr[i - 1] * atr_multiplier
        # sl_price = entry_price * (1 + sl_distance)
        tp_price = entry_price - (entry_price - price) * tp_distance
        profit_amount = entry_price - tp_price
        sl_price = entry_price + profit_amount * rr
        # tp_price = 0
        # sl_price = 999999999999999999
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


def create_signals(
    size,
    close,
    high,
    low,
    atr,
    atr_multiplier: float,
    rr: float,
    tp_distance,
    use_close=True,
):
    limit_orders = LimitOrders(len(atr))

    # limit_orders = np.zeros(((len(atr) - 15) * 2, 6))
    order_index = 0
    for i in range(15, len(atr)):
        # Place Long order
        price = close[i - 1]
        if not use_close:
            price = low[i - 1]
        entry_price = price - atr[i - 1] * atr_multiplier
        tp_price = entry_price + (price - entry_price) * tp_distance
        profit_amount = tp_price - entry_price
        sl_price = entry_price - profit_amount * rr
        # tp_price = 999999999999999999
        # sl_price = 0
        limit_orders.create_order(
            i, OrderType.Limit, Side.Long, entry_price, size, sl_price, tp_price
        )

        order_index += 1

        # Place Short order
        price = close[i - 1]
        if not use_close:
            price = high[i - 1]
        entry_price = price + atr[i - 1] * atr_multiplier
        # sl_price = entry_price * (1 + sl_distance)
        tp_price = entry_price - (entry_price - price) * tp_distance
        profit_amount = entry_price - tp_price
        sl_price = entry_price + profit_amount * rr
        # tp_price = 0
        # sl_price = 999999999999999999

        limit_orders.create_order(
            i, OrderType.Limit, Side.Short, entry_price, size, sl_price, tp_price
        )
        order_index += 1

    return limit_orders


class ATR_Strategy:
    def __init__(self, df, size, params):
        # print("===================")
        atr_multiplier, rr, tp_distance, use_close = params

        atr = talib.ATR(
            df["high"].to_numpy(),
            df["low"].to_numpy(),
            df["close"].to_numpy(),
            timeperiod=14,
        )
        # limit_orders = create_signal(
        #     size,
        #     df["close"].to_numpy(),
        #     df["high"].to_numpy(),
        #     df["low"].to_numpy(),
        #     atr,
        #     atr_multiplier,
        #     rr,
        #     tp_distance,
        #     use_close=use_close,
        # )
        # self.limit_orders = create_limit_orders(limit_orders)
        self.limit_orders = create_signals(
            size,
            df["close"].to_numpy(),
            df["high"].to_numpy(),
            df["low"].to_numpy(),
            atr,
            atr_multiplier,
            rr,
            tp_distance,
            use_close=use_close,
        )

    @staticmethod
    def generate_bt_params(simple=False):
        atr_multipler = np.arange(2, 20)
        rr = [0.3, 0.5, 1, 2]
        tp_distance = [0.33, 0.618, 0.786, 1, 1.5, 2]
        use_close = [True, False]

        if simple:
            atr_multipler = np.arange(2, 4)
            rr = [0.3, 0.5]
            tp_distance = [0.33, 0.618]
            use_close = [True]

        return list(itertools.product(atr_multipler, rr, tp_distance, use_close))
