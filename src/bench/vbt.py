import vectorbt as vbt


class VBT:
    def __init__(self, df):
        price = df
        fast_ma = vbt.MA.run(price, 10)
        slow_ma = vbt.MA.run(price, 50)
        self.entries = fast_ma.ma_crossed_above(slow_ma)
        self.exits = fast_ma.ma_crossed_below(slow_ma)
        self.price = df

    def backtest(self):
        pf = vbt.Portfolio.from_signals(
            self.price,
            self.entries,
            self.exits,
            short_entries=self.exits,
            short_exits=self.entries,
            init_cash=100,
        )
        return pf
