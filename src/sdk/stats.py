import json
from pydantic import BaseModel, ConfigDict
from decimal import Decimal
from datetime import datetime
from typing import Optional, Any
import pandas as pd
from rich import print as rprint
import numpy as np


class Position(BaseModel):
    index: Decimal
    pnl: Decimal
    pnl_pct: Decimal
    entry_timestamp: datetime
    exit_timestamp: Optional[datetime] = None
    entry_price: Decimal
    exit_price: Optional[Decimal] = None
    size: Decimal
    sl: Decimal
    tp: Decimal
    side: Decimal
    max_dd: Decimal
    close_reason: Optional[str] = None
    commission: Decimal
    commission_pct: Decimal
    drawdown: Decimal
    drawdown_pct: Decimal

    class Config:
        orm_mode = True  # Enable ORM mode for integration with ORMs if necessary
        json_encoders = {
            Decimal: lambda v: str(v)  # Ensure Decimal values are serialized as strings
        }


class Stat(BaseModel):
    initial_capital: float = 0
    pnl: float = 0
    unrealized_pnl: float = 0
    total_positions: float = 0
    closed_positions: float = 0
    active_positions: float = 0
    commissions: float = 0
    wins: float = 0
    losses: float = 0
    win_rate: float = 0

    model_config = ConfigDict(extra="allow")


class Stats(BaseModel):
    closed_positions: pd.DataFrame = pd.DataFrame()
    active_positions: pd.DataFrame = pd.DataFrame()
    equity: pd.DataFrame = pd.DataFrame()
    stats: Stat = Stat()

    class Config:
        arbitrary_types_allowed = True

    def convert_df_str_to_float(self, df):
        def is_float_string(val):
            if isinstance(val, str):
                try:
                    float(val)
                    return True  # It's a string that represents a float
                except ValueError:
                    return False  # It's a string but not a float-representing one
            return False  # It's not a string

        # Apply the function to the entire DataFrame to find float-like strings
        float_like_strings = df.applymap(is_float_string)

        for col in df.columns:
            if float_like_strings[col].any():
                df[col] = pd.to_numeric(
                    df[col], errors="coerce"
                )  # Convert to numeric (float)
        return df

    def create_equity(self, bt: Any, df: pd.DataFrame):
        equity = pd.DataFrame()
        equity.index = df.index[0 : len(bt.equity)]
        equity["realized_equity"] = np.array(bt.equity).astype(np.float64)
        equity["floating_equity"] = np.array(bt.floating_equity).astype(np.float64)
        equity["unrealized_equity"] = (
            equity["realized_equity"] + equity["floating_equity"]
        )
        self.equity = equity

    def create_positions(self, bt: Any):
        values = []
        for pos in bt.closed_positions:
            values.append(json.loads(pos.to_json()))
        self.closed_positions = self.convert_df_str_to_float(pd.DataFrame(values))

        values = []
        for pos in bt.active_positions:
            values.append(json.loads(pos.to_json()))
        self.active_positions = self.convert_df_str_to_float(pd.DataFrame(values))

    def __init__(self, bt: Any, df: pd.DataFrame):
        super().__init__()

        self.create_positions(bt)
        self.create_equity(bt, df)
        self.stats = Stat.model_validate(json.loads(bt.stats))
