# ZenBt

A python financial backtesting engine written in rust.


## Example:

Download OHLCV futures data from Binance

```python
from tradingtoolbox.exchanges import binanceklines, timeframes

klines = binanceklines()
ohlcv = klines.get_futures_klines(
    timeframes.tf_1hour, asset="btcusdt", ago="1 day ago utc"
)
```

