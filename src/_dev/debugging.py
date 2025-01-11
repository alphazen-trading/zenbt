import os
import shutil
import pandas as pd
import polars as pl
from zenbt.sdk.stats import Stats
from zenbt.sdk.trade_record import get_trades_df
from tradingtoolbox.clickhouse import ClickhouseSync


def run(directory):
    script_path = os.path.abspath(__file__)
    src = f"{directory}/strategy.py"
    dst = os.path.dirname(script_path) + "/pickl/strategy.py"
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
    # import_pickl()
    from _dev.pickl.strategy import Strategy
    from zenbt.zbt import Backtest, BacktestParams

    df = pl.read_parquet("./src/_dev/pickl/df.parquet")
    # df = df.to_pandas()
    # df["time"] = pd.to_datetime(df["time"], unit="ms")
    # df = df[-950:-400]

    # return
    st = Strategy(df, default_size=50)

    bt_params = BacktestParams(
        verbose=True,
        commission_pct=0,
        initial_capital=100_000,
        provide_active_position=True,
    )
    bt = Backtest(df, bt_params, st)

    bt.backtest()

    stats = Stats(bt, df)
    stats.print()

    trades = get_trades_df(bt)
    print(trades)
    # # print(trades.dtypes)
    # ch = ClickhouseSync.create()
    # ch.insert_df(trades, "trades")
