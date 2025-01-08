import polars as pl
import numpy as np
from numba import njit


@njit
def compute_session_max_min(session, high, low):
    n = len(session)
    session_max = 0.0
    session_min = np.inf
    maxs = np.empty(n, dtype=np.float64)
    mins = np.empty(n, dtype=np.float64)
    is_out_of_session = True

    for i in range(n):
        if session[i]:
            if is_out_of_session:
                session_max = 0.0
                session_min = np.inf
                is_out_of_session = False

            session_max = max(session_max, high[i])
            session_min = min(session_min, low[i])
        else:
            is_out_of_session = True

        maxs[i] = session_max
        mins[i] = session_min

    return maxs, mins


def create_session(
    df: pl.DataFrame,
    session_name: str,
    session_hour_start,
    session_hour_end,
    time_col_name="time",
) -> pl.DataFrame:
    # Ensure the "session_name" column exists
    if "session_name" not in df.columns:
        df = df.with_columns(pl.lit("").alias("session_name"))

    df = df.with_columns(
        (
            (
                (
                    pl.col(time_col_name).cast(pl.Datetime("ms")).dt.hour()
                    >= session_hour_start
                )
                | (
                    pl.col(time_col_name).cast(pl.Datetime("ms")).dt.hour()
                    < session_hour_end
                )
                if session_hour_start
                > session_hour_end  # For sessions crossing midnight
                else (
                    (
                        pl.col(time_col_name).cast(pl.Datetime("ms")).dt.hour()
                        >= session_hour_start
                    )
                    & (
                        pl.col(time_col_name).cast(pl.Datetime("ms")).dt.hour()
                        < session_hour_end
                    )
                )
            ).alias(session_name)
        )
    )

    # Set the session_name value based on the condition
    df = df.with_columns(
        pl.when(pl.col(session_name))  # Check if the session is True
        .then(pl.lit(session_name))  # Set the value to the session name
        .otherwise(
            pl.col("session_name")
        )  # Keep the existing value if the condition is not met
        .alias("session_name")
    )

    # Extract columns as NumPy arrays
    session_np = df[session_name].to_numpy()
    high_np = df["high"].to_numpy()
    low_np = df["low"].to_numpy()
    # Compute session max/min using Numba
    session_max_np, session_min_np = compute_session_max_min(
        session_np, high_np, low_np
    )
    df = df.with_columns(
        [
            pl.Series(f"{session_name}_max", session_max_np),
            pl.Series(f"{session_name}_min", session_min_np),
        ]
    )

    return df
