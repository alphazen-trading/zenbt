from pydantic import BaseModel
from typing import Optional
from datetime import datetime
import pandas as pd
from typing import get_args
from rich import print


class OrderRecord(BaseModel):
    id: str
    index: int
    place_timestamp: str
    fill_timestamp: str
    status: str
    client_order_id: str
    order_type: str
    side: str
    size: float
    price: float
    sl: Optional[float] = None
    tp: Optional[float] = None


def get_orders_df(bt):
    records = bt.state.orders
    orders = []

    if len(records) == 0:
        return pd.DataFrame()

    for record in records.values():
        order = OrderRecord.model_validate(record)
        orders.append(order.model_dump())

    # Create a DataFrame from the trades
    df = pd.DataFrame(orders)
    for key, value in OrderRecord.model_fields.items():
        try:
            args = get_args(value.annotation)
            if args:
                df[key] = df[key].astype(args[0])
            else:
                df[key] = df[key].astype(value.annotation)
        except Exception as e:
            pass

    df.sort_values("index", inplace=True)
    return df
