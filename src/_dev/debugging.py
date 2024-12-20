import os
import shutil
import pandas as pd
import polars as pl
from zenbt.sdk import Stats
from zenbt.sdk.trade_record import get_trades_df
from tradingtoolbox.clickhouse import ClickhouseSync


def run(directory):
    script_path = os.path.abspath(__file__)
    src = f"{directory}/strategy.py"
    dst = os.path.dirname(script_path) + "/pickl/startegy.py"
    shutil.copy(src, dst)

    src = f"{directory}/df.parquet"
    dst = os.path.dirname(script_path) + "/pickl/df.parquet"
    shutil.copy(src, dst)


def import_pickl():
    print("Start typing your input (type 'exit' to quit):")
    user_input = "/tmp/slpeters"
    run(user_input)
    while True:
        user_input = input(">> ")  # Prompt for user input
        if user_input.lower() == "exit":  # Exit condition
            print("Exiting...")
            break
        print(f"Importing from: {user_input}")  # Print the input
        run(user_input)


def test_pickl():
    from _dev.pickl.strategy import Strategy
    from zenbt.zbt import Backtest, BacktestParams

    df = pl.read_parquet("./src/_dev/pickl/df.parquet")
    st = Strategy(df, default_size=1)

    bt_params = BacktestParams(
        commission_pct=0.02 / 100,  # This is 2 bps
        initial_capital=100_000,
        provide_active_position=True,
    )
    bt = Backtest(df, bt_params, st)

    bt.backtest()

    stats = Stats(bt, df)
    stats.print()

    trades = get_trades_df(bt)
    print(trades)
    # print(trades.dtypes)
    ch = ClickhouseSync.create()
    ch.insert_df(trades, "trades")
