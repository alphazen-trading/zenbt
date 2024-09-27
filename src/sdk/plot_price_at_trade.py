from tradingtoolbox.utils import time_manip
import pandas as pd
import mplfinance as mpf
import numpy as np

from rich import print as rprint
import talib


# bbw = mpf.make_addplot(
#     c["bbw"].rolling(window=4).mean(),
#     panel=3,
#     color="orange",
#     ylabel="BBW",
# )
# subplots = [bbw]
subplots = []


class PlotPricesAtTrade:
    def plot(self, df, sub_plots, trade, line_color):
        line = [
            (pd.Timestamp(trade["open_time"]), trade["open_price"]),
            (pd.Timestamp(trade["close_time"]), trade["close_price"]),
        ]
        # color = "red" if trade["pnl_usd"] < 0 else "green"

        mpf.plot(
            df,
            style="nightclouds",
            type="candle",
            volume=False,
            title="OHLC Chart",
            addplot=subplots,
            alines=dict(alines=[line], colors=[line_color], linewidths=[2]),
        )

    def prepare(self, ohlc, trade, funding, left_delta, right_delta):
        _time_delta = pd.to_timedelta(left_delta, unit="m")
        open = pd.to_datetime(trade["open_time"]) - _time_delta
        _time_delta = pd.to_timedelta(right_delta, unit="m")
        close = pd.to_datetime(trade["close_time"]) + _time_delta
        c = ohlc[ohlc["date"] > open]
        c = c[c["date"] < close]
        c.set_index("date", inplace=True)

        f = funding[funding["date"] > open]
        f = f[f["date"] < close]
        f.set_index("date", inplace=True)

        c["rsi"] = talib.RSI(c["close"].to_numpy(), timeperiod=14)
        c["ema"] = talib.EMA(c["close"].to_numpy(), timeperiod=50)
        c["ema_2"] = talib.EMA(c["close"].to_numpy(), timeperiod=200)
        upper, middle, lower = talib.BBANDS(c["close"].to_numpy())
        c["bbw"] = (upper - lower) / middle
        return c, f
        # rprint(rsi_value)

    def run(self):
        df = pd.read_parquet("./data/wallet_events.parquet")
        df["open_time"] = pd.to_datetime(df["open_time"])
        df["close_time"] = pd.to_datetime(df["close_time"])
        df["duration (h)"] = (
            (df["close_time"] - df["open_time"]).dt.total_seconds() / 60 / 60
        )
        rprint("Borrow fee: ", df["borrow_fee_usd"].sum())
        rprint("Crank fee: ", df["crank_fee_usd"].sum())
        rprint("Funding fee: ", df["funding_fee_usd"].sum())
        rprint("DNF fee: ", df["dnf_fee_usd"].sum())
        rprint("Trading fee: ", df["trading_fee_usd"].sum())

        df = df[df["is_active"] == 0]
        df["fees"] = (
            df["borrow_fee_usd"]
            + df["crank_fee_usd"]
            + df["dnf_fee_usd"]
            + df["funding_fee_usd"]
            + df["trading_fee_usd"]
        )
        df.drop(
            columns=[
                "contract",
                "notional_size_in_collateral",
                "deposit_collateral_in_usd",
                "deposit_collateral",
                "deposit_collateral",
                "open_time",
                "close_time",
                "pnl",
                # "pnl_usd",
                "notional_size",
                "direction",
                "open_price",
                "close_price",
                "open_price_q",
                #
                # "trading_fee_usd",
                # "crank_fee_usd",
                # "dnf_fee_usd",
                # "funding_fee_usd",
                # "borrow_fee_usd",
                #
                "pos_id",
                # "contract_name",
                # "close_reason",
                # "borrow_fee_usd",
                "is_active",
                "pnl",
            ],
            inplace=True,
        )

        df["pnl_usd"] = df["pnl_usd"] - df["fees"]
        # df = df[df["duration (h)"] < 1]
        # df["capacity"] = np.abs(df["fees"]) / (
        #     np.abs(df["fees"]) + np.abs(df["pnl_usd"])
        # )
        # df.sort_values(by="capacity", ascending=False, inplace=True)
        # df = df[df["capacity"] > 0.2]
        # rprint(df)
        # rprint(df["pnl_usd"].sum())
        # df = df[df["pnl_usd"] < 0]
        # rprint(df)
        df["funding_fee_usd"] = df["funding_fee_usd"]
        # df["diff"] = df["funding_fee_usd"] > (df["trading_fee_usd"] + df["dnf_fee_usd"])
        df.sort_values(by="duration (h)", ascending=False, inplace=True)
        # df = df[df["diff"] == True]
        rprint(df[df["fees"] > 0].head(20))
        print()
        rprint(df[df["fees"] < 0].head(20))
        return
        wins = len(df[df["pnl_usd"] > 0])
        losses = len(df[df["pnl_usd"] < 0])
        # rprint(wins / (wins + losses))

        # rprint(1 - len(df[df["close_reason"] == "direct"]) / len(df))
        rprint(df["pnl_usd"].sum())
        rprint(df["fees"].sum())
        rprint(df["fees"].sum() / df["pnl_usd"].sum())
        # rprint(df.head(20))

    def run_plot(self):
        asset = "SOL_USDC"
        left_delta = 300
        right_delta = 100
        funding = self.get_funding_rates(asset)
        # self.fetch_ohlc(asset)
        ohlc = pd.read_parquet(f"./data/ohlc/{asset}.parquet")
        ohlc.set_index("date", inplace=True)
        ohlc.sort_index(ascending=True, inplace=True)
        ohlc.reset_index(inplace=True)

        df = pd.read_parquet("./data/wallet_events.parquet")

        df["open_time"] = pd.to_datetime(df["open_time"])
        df["close_time"] = pd.to_datetime(df["close_time"])
        df["open_time"] = pd.to_datetime(df["open_time"]).dt.strftime(
            "%Y-%m-%d %H:%M:%S"
        )
        df["close_time"] = pd.to_datetime(df["close_time"]).dt.strftime(
            "%Y-%m-%d %H:%M:%S"
        )
        df = df[df["is_active"] == 0]
        df.drop(
            columns=[
                "contract",
                "notional_size_in_collateral",
                "deposit_collateral_in_usd",
                "deposit_collateral",
                "deposit_collateral",
                # "open_time",
                # "close_time",
                "pnl",
                # "trading_fee_usd",
                # "crank_fee_usd",
                # "dnf_fee_usd",
                # "funding_fee_usd",
                # "borrow_fee_usd",
                "is_active",
                "pnl",
            ],
            inplace=True,
        )

        df = df[df["contract_name"] == asset]
        df.sort_values(by="pnl_usd", ascending=False, inplace=True)

        values = []
        for i in range(len(df)):
            trade = df.iloc[i]

            c, f = self.prepare(ohlc, trade, funding, left_delta, right_delta)

            rsi_value = c[c.index < trade["open_time"]].iloc[-1]["rsi"]
            ema_value = c[c.index < trade["open_time"]].iloc[-1]["ema"]
            bbw_value = c[c.index < trade["open_time"]].iloc[-1]["bbw"]
            values.append(
                [
                    trade["direction"],
                    rsi_value,
                    ema_value,
                    bbw_value,
                    trade["open_price"],
                    trade["open_price"] > ema_value,
                    trade["pnl_usd"],
                    trade["close_reason"],
                ]
            )

        new_df = pd.DataFrame(
            values,
            columns=[
                "direction",
                "rsi",
                "ema",
                "bbw",
                "close",
                "higher",
                "pnl_usd",
                "close-reason",
            ],
        )
        # rprint(new_df)

        ddf = new_df[new_df["direction"] == "short"]
        ddf.sort_values(by="bbw", ascending=False, inplace=True)
        rprint(ddf)
        # pnl_u = ddf[ddf["rsi"] > 50]["pnl_usd"].sum()
        # pnl_d = ddf[ddf["rsi"] < 50]["pnl_usd"].sum()
        # print(pnl_u)
        # print(pnl_d)

        trade = df.iloc[16]

        trade["open_time"] = pd.to_datetime(trade["open_time"])
        trade["close_time"] = pd.to_datetime(trade["close_time"])
        c, f = self.prepare(ohlc, trade, funding, left_delta, right_delta)

        rsi_value = c[c.index < trade["open_time"]].iloc[-1]["rsi"]
        ema_value = c[c.index < trade["open_time"]].iloc[-1]["ema"]
        self.plot(c, f, trade, show_indicators=True)
        rprint(trade["open_time"], trade["direction"], rsi_value, ema_value)
        # rprint(trade["funding_fee_usd"], trade["trading_fee_usd"], trade["pnl_usd"])
        # rprint(trade["close_time"] - trade["open_time"])
        trade["fee"] = (
            trade["funding_fee_usd"]
            + trade["trading_fee_usd"]
            + trade["borrow_fee_usd"]
            + trade["crank_fee_usd"]
            + trade["dnf_fee_usd"]
        )
        # rprint(trade["fee"], trade["pnl_usd"])
        # rprint(trade)
        # rprint(help(talib.BBANDS))
        rprint(140 / 139)


PlotPricesAtTrade().run_plot()
