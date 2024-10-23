from typing import Final

class Action:
    @property
    def desired_orders(self): ...
    @property
    def desired_positions(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Action: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class BBO:
    @property
    def ask_price(self): ...
    @property
    def ask_size(self): ...
    @property
    def bid_price(self): ...
    @property
    def bid_size(self): ...
    @property
    def print(self): ...
    @property
    def time(self): ...
    def datetime(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> BBO: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class Backtest:
    @property
    def backtest_params(self): ...
    @property
    def data(self): ...
    @property
    def df(self): ...
    @property
    def state(self): ...
    @property
    def strategy(self): ...
    def backtest(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Backtest: ...
    def _set_state_dict_item(self, dict_name, key, value): ...
    def _append_to_list(self, list_name, value): ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class BacktestOld:
    @property
    def commissions(self): ...
    @property
    def equity(self): ...
    @property
    def floating_equity(self): ...
    @property
    def limit_orders(self): ...
    @property
    def ohlc(self): ...
    @property
    def params(self): ...
    @property
    def positions(self): ...
    @property
    def trailing_tp(self): ...
    def get_stats(self): ...
    def get_state(self): ...
    def backtest_signals(self, long_entries, long_exits, short_entries, short_exits): ...
    def backtest(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> BacktestOld: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class BacktestParams:
    @staticmethod
    def __new__(*args, **kwargs) -> BacktestParams: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class Contract:
    @property
    def code(self): ...
    @property
    def exchange_base_underlying(self): ...
    @property
    def exchange_counter_underlying(self): ...
    @property
    def id(self): ...
    @property
    def is_active(self): ...
    @property
    def last_price(self): ...
    @property
    def min_order(self): ...
    @property
    def min_order_usd_value(self): ...
    @property
    def tick_size(self): ...
    def print(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Contract: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class LimitOrders:
    def get(self, index): ...
    def create_order(self, index, order_type, side, price, size, sl, tp): ...
    @staticmethod
    def __new__(*args, **kwargs) -> LimitOrders: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class OHLCs:
    @staticmethod
    def __new__(*args, **kwargs) -> OHLCs: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class Order:
    @property
    def clientOrderId(self): ...
    @property
    def index(self): ...
    @property
    def order_type(self): ...
    @property
    def price(self): ...
    @property
    def side(self): ...
    @property
    def size(self): ...
    @property
    def sl(self): ...
    @property
    def tp(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Order: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class OrderType:
    Limit: Final[OrderType]
    Market: Final[OrderType]
    @staticmethod
    def __new__(*args, **kwargs) -> OrderType: ...
    def __lt__(self, value): ...
    def __int__(self): ...
    def __init__(self, *args, **kwargs): ...
    def __eq__(self, value): ...

class Position:
    @property
    def side(self): ...
    def was_tp_hit(self, i, ohlc): ...
    def was_sl_hit(self, i, ohlc): ...
    def update_pnl(self, close): ...
    def should_close(self, i, ohlc): ...
    def close_position(self, i, ohlc, exit_price, close_reason, pnl): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Position: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class SharedState:
    @property
    def active_positions(self): ...
    @property
    def closed_positions(self): ...
    @property
    def equity(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> SharedState: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class Side:
    Long: Final[Side]
    Short: Final[Side]
    @staticmethod
    def __new__(*args, **kwargs) -> Side: ...
    def __lt__(self, value): ...
    def __int__(self): ...
    def __init__(self, *args, **kwargs): ...
    def __eq__(self, value): ...

class Signal:
    @property
    def index(self): ...
    @property
    def order_type(self): ...
    @property
    def side(self): ...
    @property
    def signal_type(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Signal: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class Signals:
    @property
    def print(self): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Signals: ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class Strategy:
    @property
    def data(self): ...
    @property
    def df(self): ...
    @property
    def equity(self): ...
    @property
    def floating_equity(self): ...
    @property
    def positions(self): ...
    @staticmethod
    def on_candle(): ...
    @staticmethod
    def __new__(*args, **kwargs) -> Strategy: ...
    @staticmethod
    def _on_candle(): ...
    def __lt__(self, value): ...
    def __init__(self, *args, **kwargs): ...
    def __hash__(self): ...
    def __eq__(self, value): ...

class cross_above:
    @staticmethod
    def __new__(*args, **kwargs) -> cross_above: ...
    @staticmethod
    def __lt__(value): ...
    @staticmethod
    def __init__(*args, **kwargs): ...
    @staticmethod
    def __hash__(): ...
    @staticmethod
    def __eq__(value): ...
    @staticmethod
    def __call__(*args, **kwargs): ...

class cross_below:
    @staticmethod
    def __new__(*args, **kwargs) -> cross_below: ...
    @staticmethod
    def __lt__(value): ...
    @staticmethod
    def __init__(*args, **kwargs): ...
    @staticmethod
    def __hash__(): ...
    @staticmethod
    def __eq__(value): ...
    @staticmethod
    def __call__(*args, **kwargs): ...

