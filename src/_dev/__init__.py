from .debugging import import_pickl
from .debugging import test_pickl


def dev():
    # from zenbt.strategies import MaCross
    # from zenbt.data import get_sample_btc_data
    # from zenbt.sdk.stats import Stats
    # from zenbt.zbt import Backtest, indicators
    # import talib
    # df = get_sample_btc_data()

    from .mt5 import dev as _dev
    # from .dev import dev as _dev
    # from .indicator import dev as _dev

    _dev()
