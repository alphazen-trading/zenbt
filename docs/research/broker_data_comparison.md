# Broker Data & API Comparison: Interactive Brokers vs Alpaca

*Last updated: January 2026*

This document compares the data offerings, authentication requirements, and practical considerations for Interactive Brokers and Alpaca - two popular brokers for algorithmic trading.

---

## Quick Summary

| Feature | Interactive Brokers | Alpaca (Free) | Alpaca ($99/mo) |
|---------|---------------------|---------------|-----------------|
| **Historical data depth** | Up to 20 years (daily) | Since 2016 | Since 2016 |
| **1H bars available** | ~1 month per request | Since 2016 | Since 2016 |
| **4H bars** | Via resample | Via resample | Via resample |
| **Real-time data** | Paid subscriptions | IEX only (2.5%) | All US exchanges |
| **Auth complexity** | High (TWS/Gateway) | Low (API keys) | Low (API keys) |
| **Weekly restart needed** | Yes (mandatory) | No | No |
| **Rate limits** | ~60 req/10 min | 200 req/min | 10,000 req/min |
| **Recent data delay** | None (with subscription) | 15-min delay | No delay |

---

## Part 1: Interactive Brokers

### Connection Architecture

```
Your Python Code (ib_async/lumibot)
        |
   TCP Socket Connection
        |
TWS or IB Gateway (running locally or on server)
        |
   Internet
        |
Interactive Brokers Servers
```

**Two ways to connect:**

| Component | Description | Port | Best For |
|-----------|-------------|------|----------|
| **TWS (Trader Workstation)** | Full desktop trading platform with GUI | 7497 (live), 7496 (paper) | Manual trading + API |
| **IB Gateway** | Headless, lightweight, API-only | 4001 (live), 4002 (paper) | Automated trading bots |

### Python Libraries for IB

| Library | Stars | Description |
|---------|-------|-------------|
| [ib_async](https://github.com/ib-api-reloaded/ib_async) | 1.3k | Modern async IB API (replaces ib_insync) |
| [Lumibot](https://github.com/Lumiwealth/lumibot) | 1.2k | High-level framework with IB support |

Both connect the same way:
```python
# ib_async
from ib_async import IB
ib = IB()
ib.connect('127.0.0.1', 7497, clientId=1)  # TWS
ib.connect('127.0.0.1', 4001, clientId=1)  # Gateway

# Lumibot uses ib_insync internally (same approach)
```

### Authentication & Weekly Restart

**Critical limitation: IB Gateway has a mandatory weekly restart for security.**

| Scenario | Re-authentication Required? | Notes |
|----------|----------------------------|-------|
| **TWS Desktop** | Every 24 hours OR weekly | Configurable in settings |
| **IB Gateway** | Weekly (Sunday) | Mandatory security restart |
| **Paper Trading** | Same as above | Uses separate credentials |

**Can it be automated?** Yes, with IBC (IB Controller):

```
Sunday (scheduled time)
    |
IB Gateway auto-restarts (IB forces this)
    |
IBC detects restart, re-launches Gateway
    |
IBC auto-enters credentials
    |
IBC handles 2FA via IBKR Mobile "trust this device"
    |
Your bot reconnects automatically
```

**Popular Docker images for automation:**

| Image | Stars | Notes |
|-------|-------|-------|
| [gnzsnz/ib-gateway-docker](https://github.com/gnzsnz/ib-gateway-docker) | 300+ | Most maintained, includes IBC |
| [UnusualAlpha/ib-gateway-docker](https://github.com/UnusualAlpha/ib-gateway-docker) | 200+ | Alternative |

**Docker example:**
```bash
docker run -d \
  -e TWSUSERID=your_username \
  -e TWSPASSWORD=your_password \
  -p 4001:4001 \
  ghcr.io/gnzsnz/ib-gateway:stable
```

**Reconnection pattern in code:**
```python
async def run():
    while True:
        try:
            await ib.connectAsync('127.0.0.1', 4001, clientId=1)
            # ... your trading logic
        except Exception as e:
            print(f"Disconnected: {e}, reconnecting in 60s...")
            await asyncio.sleep(60)
```

### Market Data: Free vs Paid

**Free Data (No Subscription Required):**

| Data Type | What You Get | Limitation |
|-----------|--------------|------------|
| **Delayed quotes** | 10-20 min delay | Free for most markets |
| **Free US streaming** | Cboe One + IEX | Non-consolidated (not NBBO) |
| **100 snapshot quotes/month** | Real-time, on-demand | $0.01-0.03 per additional quote |
| **Historical data** | Via API | FREE - no subscription needed |

**To get delayed data in code:**
```python
ib.reqMarketDataType(3)  # Delayed
ib.reqMarketDataType(4)  # Delayed frozen
ib.reqMarketDataType(1)  # Real-time (requires subscription)
```

**Paid Subscriptions for Real-Time Data:**

| Market | Non-Pro Price | Pro Price | What You Get |
|--------|---------------|-----------|--------------|
| **US Stocks Bundle** | $10/mo | $10/mo + $0.01/snapshot | Consolidated NBBO (NYSE, NASDAQ, AMEX) |
| **US Streaming Add-on** | $4.50/mo | $125/mo | Streaming instead of snapshots |
| **OPRA (Options)** | $1.50/mo | $32.75/mo | All US options exchanges |
| **NYSE Level 1** | $1.50/mo | $45/mo | NYSE top of book |
| **NASDAQ Level 1** | $1.50/mo | $25/mo | NASDAQ top of book |
| **CME/CBOT/NYMEX** | $12.10/mo each | $145/mo each | Futures data |

**Commission Waivers:**
- Some data subscriptions are waived if you generate enough commissions
- Example: US Securities Bundle ($10) waived if you do $30/mo in commissions

### Historical Data Limits

| Bar Size | Maximum Duration per Request | Approximate Data |
|----------|------------------------------|------------------|
| 1 sec | 2,000 seconds | ~33 minutes |
| 1 min | 1 day | 1 day |
| 5 min | 1 week | 1 week |
| 15 min | 2 weeks | 2 weeks |
| 30 min | 1 month | 1 month |
| **1 hour** | **1 month** | **~720 bars** |
| **4 hours** | **1 month** | **~180 bars** |
| 1 day | 1 year | ~252 bars |
| 1 week | 5 years | ~260 bars |
| 1 month | 20 years | ~240 bars |

**Getting more data:** Make multiple requests going backwards in time:

```python
from datetime import datetime, timedelta

contract = Stock('AAPL', 'SMART', 'USD')
all_bars = []

# Request in 1-month chunks going backwards
end_date = datetime.now()
for i in range(12):  # 12 months
    bars = ib.reqHistoricalData(
        contract,
        endDateTime=end_date.strftime('%Y%m%d %H:%M:%S'),
        durationStr='1 M',
        barSizeSetting='4 hours',
        whatToShow='TRADES',
        useRTH=False
    )
    all_bars = bars + all_bars
    end_date = end_date - timedelta(days=30)
    ib.sleep(1)  # Avoid pacing violations
```

**Pacing Limits:**
- IB limits ~60 requests per 10 minutes
- Add `ib.sleep(1)` between requests to avoid "pacing violation" errors

---

## Part 2: Alpaca

### Authentication

**Much simpler than IB - just API keys:**

```python
from alpaca.data import StockHistoricalDataClient

client = StockHistoricalDataClient(
    api_key="your_api_key",
    secret_key="your_secret_key"
)
```

- No TWS/Gateway to run
- No weekly restarts
- No 2FA handling needed
- Works from any environment (cloud, local, etc.)

### Subscription Plans

| Feature | Basic (Free) | Algo Trader Plus ($99/mo) |
|---------|--------------|---------------------------|
| **Price** | Free | $99/month |
| **Historical data** | Since 2016 | Since 2016 |
| **Historical limitation** | Latest 15 min excluded | No restriction |
| **Real-time coverage** | IEX only (~2.5% volume) | All US exchanges |
| **WebSocket symbols** | 30 | Unlimited |
| **API calls** | 200/min | 10,000/min |

### Data Sources

| Source | Description | Free Plan |
|--------|-------------|-----------|
| **iex** | Single exchange (~2.5% volume) | Yes |
| **sip** | All US exchanges (consolidated) | Paid only |
| **boats** | Blue Ocean ATS (overnight trading) | Paid only |

### Available Bar Sizes

| Bar Size | Available |
|----------|-----------|
| 1 min | Yes |
| 5 min | Yes |
| 15 min | Yes |
| 30 min | Yes |
| **1 hour** | **Yes** |
| **4 hour** | **Via resample from 1H** |
| 1 day | Yes |
| 1 week | Yes |
| 1 month | Yes |

### Historical Data Depth

**For free accounts:**
- Data goes back to **2016** (8+ years!)
- No duration limits per request - just pagination
- Only limitation: Can't access the most recent 15 minutes

### Code Examples

**Get 1-hour bars (FREE):**
```python
from alpaca.data import StockHistoricalDataClient
from alpaca.data.requests import StockBarsRequest
from alpaca.data.timeframe import TimeFrame
from datetime import datetime

client = StockHistoricalDataClient(api_key, secret_key)

request = StockBarsRequest(
    symbol_or_symbols="AAPL",
    timeframe=TimeFrame.Hour,
    start=datetime(2016, 1, 1),
    end=datetime(2024, 1, 1)
)

bars = client.get_stock_bars(request)
# Returns ~8 years of hourly data!
```

**Get 4-hour bars via resample:**
```python
import pandas as pd

# Get 1H bars
bars_1h = client.get_stock_bars(StockBarsRequest(
    symbol_or_symbols="AAPL",
    timeframe=TimeFrame.Hour,
    start=datetime(2020, 1, 1)
))

# Convert to DataFrame and resample to 4H
df = bars_1h.df
df_4h = df.resample('4H').agg({
    'open': 'first',
    'high': 'max', 
    'low': 'min',
    'close': 'last',
    'volume': 'sum'
})
```

---

## Part 3: Comparison for Common Use Cases

### For Backtesting (Historical Data)

| Criteria | Winner | Notes |
|----------|--------|-------|
| **Ease of setup** | Alpaca | Just API keys, no Gateway |
| **Data depth** | Tie | Both have years of data |
| **Rate limits** | Alpaca | 200/min vs IB's ~60/10min |
| **Cost** | Tie | Both free for historical |
| **1H/4H data** | Alpaca | 8+ years vs IB's chunked requests |

**Recommendation:** Alpaca for backtesting historical data

### For Live Trading

| Criteria | Winner | Notes |
|----------|--------|-------|
| **Real-time data quality** | IB | Consolidated NBBO with subscription |
| **Broker integration** | Tie | Both work well |
| **Options trading** | IB | More mature |
| **Crypto trading** | Alpaca | Native support |
| **Uptime/reliability** | Alpaca | No weekly restarts |

**Recommendation:** Depends on asset class and real-time needs

### For a Typical Retail Algo Trader

**Minimal cost setup:**
- **Backtesting:** Alpaca Free (historical data since 2016)
- **Live trading:** 
  - Alpaca Free if delayed data is OK
  - IB with $10-15/mo subscriptions for real-time

---

## Part 4: Key Takeaways

### Interactive Brokers
1. **Connection**: TCP socket to TWS/Gateway running locally
2. **Re-authentication**: Required weekly (Sunday) - use IBC + Docker for automation
3. **Free data**: Delayed quotes, 100 snapshots/month, free streaming from Cboe One/IEX
4. **Paid data**: $10-15/month for consolidated US real-time
5. **Historical data**: FREE via API, but limited duration per request
6. **Professional rates**: 5-10x higher if you're in finance professionally

### Alpaca
1. **Connection**: Simple REST API with API keys
2. **Re-authentication**: None needed
3. **Free data**: IEX only for real-time, but full historical since 2016
4. **15-min delay**: Free tier can't access most recent 15 minutes
5. **Rate limits**: 200 req/min (free) vs 10,000 req/min (paid)
6. **Best for**: Backtesting with long historical data

### For Historical 1H/4H Data Specifically

**Alpaca Free is the clear winner:**
- 8+ years of data (since 2016)
- No complex auth (just API keys)
- No pacing violations
- No chunked requests needed
- Only catch: 15-minute delay on most recent data (doesn't matter for backtesting)

---

## Links

### Interactive Brokers
- ib_async: https://github.com/ib-api-reloaded/ib_async
- Lumibot: https://github.com/Lumiwealth/lumibot
- IBC (auto-restart): https://github.com/IbcAlpha/IBC
- Docker Gateway: https://github.com/gnzsnz/ib-gateway-docker
- Market Data Pricing: https://www.interactivebrokers.com/en/pricing/research-news-marketdata.php

### Alpaca
- Documentation: https://docs.alpaca.markets/
- Python SDK: https://github.com/alpacahq/alpaca-py
- Market Data API: https://docs.alpaca.markets/docs/about-market-data-api
- Pricing: https://alpaca.markets/data
