import pandas as pd
from datetime import datetime
import time
from tqdm import tqdm


def multi_backtest(df, bt, size, params, bt_method):
    start = time.time()
    stats = []
    for param in tqdm(params):
        stats.append(bt_method(df, bt, size, param))
    end = time.time()

    dicts = [obj.dict() for obj in stats]
    df = pd.DataFrame(dicts, index=params)
    print(f"Backtested {len(params)} combinations in {end - start} seconds")
    print(df)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    filename = f"./data/simulation_result_{timestamp}.parquet"
    df.to_parquet(filename)
