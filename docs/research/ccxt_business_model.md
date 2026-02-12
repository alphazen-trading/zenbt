# CCXT Business Model Analysis

*Last updated: January 2026*

## Overview

**CCXT (CryptoCurrency eXchange Trading Library)** is a 100% free and open-source library (MIT license) for connecting to cryptocurrency exchanges. Despite being free, they have a sustainable business model.

---

## Revenue Streams

| Revenue Source                  | Description                                                                                           |
| ------------------------------- | ----------------------------------------------------------------------------------------------------- |
| **Exchange API Broker Rebates** | Primary revenue - CCXT receives commissions when users trade via exchanges using their referral codes |
| **Open Collective Donations**   | ~$25,000 total raised, ~$3,700/year                                                                   |
| **GitHub Sponsors**             | Corporate and individual sponsorships                                                                 |

---

## Key Facts

| Metric              | Value                                       |
| ------------------- | ------------------------------------------- |
| GitHub Stars        | 40,400+                                     |
| Forks               | 8,400+                                      |
| Supported Exchanges | 108+                                        |
| Languages           | JavaScript, TypeScript, Python, PHP, C#, Go |
| License             | MIT (completely free)                       |

---

## CCXT Pro History

- **Before October 2022:** CCXT Pro was a paid subscription at **$29/month** for WebSocket functionality
- **After October 2022:** CCXT Pro was merged into the free CCXT package (v1.95+)
- **Current:** All functionality including WebSockets is **completely free**

The decision to make Pro free suggests exchange rebates generate sufficient revenue.

---

## Referral Programs vs Broker Programs

CCXT uses **two different revenue mechanisms** - it's important to understand the difference:

### Comparison

| Aspect              | Referral Program              | Broker Program                                |
| ------------------- | ----------------------------- | --------------------------------------------- |
| **Target**          | Individual users              | Businesses/Platforms                          |
| **How it works**    | User signs up via link        | Broker code embedded in API calls             |
| **Commission on**   | New user signups only         | ALL trades via broker API (new + existing)    |
| **Technical needs** | None (just share link)        | API integration with broker code              |
| **Typical rates**   | 10-30% of referee's fees      | 20-70% of trading fees                        |
| **Disclosure**      | Visible (user clicks link)    | Often hidden (embedded in library)            |

---

## 1. Referral Program (User Sign-ups)

These are the public referral links in CCXT's README for new user sign-ups:

**Source:** [CCXT Official GitHub README](https://github.com/ccxt/ccxt)

| Exchange       | User Discount | Referral Link                                     |
| -------------- | ------------- | ------------------------------------------------- |
| Crypto.com     | 75%           | `https://crypto.com/exch/kdacthrnxt`              |
| WOO X          | 35%           | `https://woox.io/register?ref=DIJT0CNL`           |
| BitMart        | 30%           | `http://www.bitmart.com/?r=rQCFLh`                |
| OKX            | 20%           | `https://www.okx.com/join/CCXTCOM`                |
| Gate.io        | 20%           | `https://www.gate.com/share/CCXTGATE`             |
| HTX (Huobi)    | 15%           | `https://www.htx.com.vc/invite/en-us/1h?invite_code=6rmm2223` |
| Binance        | 10%           | `https://accounts.binance.com/register?ref=CCXTCOM` |
| BitMEX         | 10%           | `https://www.bitmex.com/app/register/NZTR1q`      |
| WOOFI PRO      | 5%            | `https://dex.woo.org/en/trade?ref=CCXT`           |
| Hyperliquid    | 4%            | First $25M volume (Builder Codes program)         |

---

## 2. Broker Program (API Trading Commission)

This is CCXT's **primary revenue source** - broker codes embedded directly in the library.

### How It Works

CCXT embeds broker codes in API request headers/parameters:

```python
# Example from CCXT's Bybit implementation
if method == 'POST':
    brokerId = self.safe_string(self.options, 'brokerId')
    if brokerId is not None:
        headers['Referer'] = brokerId
```

Every trade made through CCXT generates commission for CCXT - **regardless of whether the user signed up via their referral link**.

### Exchange Broker Commission Rates (General Industry Rates)

**Note:** These are general broker program rates from exchange documentation. CCXT does not publicly disclose what specific rates they receive.

**Sources:** Exchange broker program pages

| Exchange    | Broker Commission | Settlement | Source                              |
| ----------- | ----------------- | ---------- | ----------------------------------- |
| KuCoin      | Up to **70%**     | Varies     | https://www.kucoin.com/broker       |
| HTX (Huobi) | Up to **65%**     | Varies     | https://www.htx.com/en-us/broker    |
| OKX         | Up to **50%**     | T+1 hourly | https://www.okx.com/broker          |
| Binance     | Up to **50%**     | T+1        | Binance Link program                |
| Bybit       | ~50%              | Varies     | Bybit broker program                |

*Actual rates depend on volume tiers and partnership agreements.*

### Evidence from Primary Sources

**1. CCXT README (Official Statement):**
> "**API broker** means CCXT is funded with rebates from exchanges' API broker programs and it is an official API broker with many exchanges, all rebates and related fees are handled by the exchanges solely in accordance with exchanges' respective terms and conditions established by each partner exchange."

Source: https://github.com/ccxt/ccxt (Disclaimer section)

**2. GitHub Issue #26078** - "CCXT takes referral fee without any notice?"
- Users discovered CCXT embeds `brokerId` in API request headers
- Code shows: `headers['Referer'] = brokerId`
- Source: https://github.com/ccxt/ccxt/issues/26078

**3. User reports (unverified claims from issue):**
- Bybit: ~50% of user's commission rebates allegedly go to CCXT
- Hyperliquid & KuCoin: May take all rebates

### Key Insight

> CCXT earns commission on **every trade** made through their library, not just from users who signed up via referral links. This is why they could make CCXT Pro free - the broker commissions alone are substantial.

---

## Complete Broker Code Analysis (Source Code Verified)

**Analysis Date:** January 2026  
**Source:** CCXT GitHub Repository (`python/ccxt/*.py`)  
**Total Exchanges with Broker Codes:** 28 out of 111 supported exchanges

### All CCXT Broker Codes

| # | Exchange | Broker Code | Injection Method | Header/Param |
|---|----------|-------------|------------------|--------------|
| 1 | **Binance** (Spot/Margin) | `x-TKT5PX2F` | Client Order ID prefix | `newClientOrderId` |
| 2 | **Binance** (Futures) | `x-cvBPrNm9` | Client Order ID prefix | `newClientOrderId` |
| 3 | **Binance** (Delivery/Options) | `x-xcKtGhcu` | Client Order ID prefix | `newClientOrderId` |
| 4 | **Bybit** | `CCXT` | HTTP Header | `Referer` |
| 5 | **OKX** | `6b9ad766b55dBCDE` | Request params | `clOrdId`, `tag` |
| 6 | **BitMart** | `CCXTxBitmart000` | HTTP Header | `X-BM-BROKER-ID` |
| 7 | **CoinEx** | `x-167673045` | Options config | `brokerId` |
| 8 | **BingX** | `CCXT` | HTTP Header | `X-SOURCE-KEY` |
| 9 | **Bitget** | `p4sve` | Options config | `broker` |
| 10 | **HTX (Huobi)** | `AA03022abc` | Options config | `broker.id` |
| 11 | **KuCoin** (Spot) | `ccxt` + key | Partner header | `KC-API-PARTNER` |
| 12 | **KuCoin** (Futures) | `ccxtfutures` + key | Partner header | `KC-API-PARTNER` |
| 13 | **MEXC** | `CCXT` | Request param | `source` |
| 14 | **Crypto.com** | `CCXT` | Request param | `broker_id` |
| 15 | **Phemex** | `CCXT123456` | Request param | `text` |
| 16 | **WhiteBIT** | `ccxt` | Options config | `brokerId` |
| 17 | **WOO X** | `bc830de7-50f3-460b-9ee0-f430f83f9dad` | Request param | `broker_id` |
| 18 | **WOOFi Pro** | `CCXT` | Request param | `order_tag` |
| 19 | **BitMEX** | `CCXT` | Request param | `text` |
| 20 | **Coinbase** | `ccxt` | Client Order ID prefix | `client_order_id` |
| 21 | **Coinbase Intl** | `nfqkvdjp` | Client Order ID prefix | `client_order_id` |
| 22 | **BloFin** | `ec6dd3a7dd982d0b` | Request param | `brokerId` |
| 23 | **Apex** | `6956` | Request param | `brokerId` |
| 24 | **CoinCatch** | `47cfy` | HTTP Header | `X-CHANNEL-API-CODE` |
| 25 | **HashKey** | `10000700011` | HTTP Header | `INPUT-SOURCE` |
### CeFi vs DeFi Breakdown

Of the 28 exchanges with codes, **6 are DEXes** with different monetization mechanisms:

| # | Exchange | Type | Code | Program Type |
|---|----------|------|------|--------------|
| 23 | **Apex** | DEX | `6956` | Broker ID |
| 26 | **Hyperliquid** | DEX | `0x6530512A6c89C7cfCEbC3BA7fcD9aDa5f30827a6` | Builder Code |
| 27 | **ModeTrade** | DEX | `CCXTMODE` | Broker ID |
| 28 | **Paradex** | DEX | `CCXT` | Partner Header |
| 18 | **WOOFi Pro** | DEX | `CCXT` | Broker ID |
| - | **Derive** | DEX | `0x0ad42b8e602c2d3d475ae52d678cf63d84ab2749` | Referral Code |

---

## DeFi Builder/Referral Codes (Detailed)

DeFi protocols use different mechanisms than CeFi broker programs. Key differences:

| Aspect | CeFi Broker | DeFi Builder/Referral |
|--------|-------------|----------------------|
| **User Consent** | Implicit | Often explicit (user signs approval) |
| **Transparency** | Hidden in headers | On-chain, fully visible |
| **Commission** | 20-70% of fees | 0.01-1% of trade value |
| **Settlement** | Off-chain, periodic | On-chain, per-trade |

### Hyperliquid Builder Code

**Most sophisticated DeFi program in CCXT**

| Feature | Value |
|---------|-------|
| **Builder Address** | `0x6530512A6c89C7cfCEbC3BA7fcD9aDa5f30827a6` |
| **Max Fee (Perps)** | 0.1% (10 basis points) |
| **Max Fee (Spot)** | 1% (100 basis points) |
| **User Consent** | Required - must sign `ApproveBuilderFee` |
| **Referral Code** | `CCXT1` (separate program, 10% of fees) |

```python
# Hyperliquid builder fee implementation
orderAction['builder'] = {
    'b': '0x6530512A6c89C7cfCEbC3BA7fcD9aDa5f30827a6',  # Builder wallet
    'f': 10  # Fee in tenths of bps (10 = 0.01%)
}

# Separate referral code
action = {'type': 'setReferrer', 'code': 'CCXT1'}
```

### Apex Pro Builder Code

| Feature | Value |
|---------|-------|
| **Broker ID** | `6956` |
| **Implementation** | Standard broker ID in request |

### WOOFi Pro / ModeTrade (Orderly Network)

Both built on Orderly Network infrastructure:

| Exchange | Broker ID | Key Broker ID |
|----------|-----------|---------------|
| **WOOFi Pro** | `CCXT` | `woofi_pro` |
| **ModeTrade** | `CCXTMODE` | `mode` |

### Paradex

| Feature | Value |
|---------|-------|
| **Partner Header** | `PARADEX-PARTNER: CCXT` |
| **Type** | StarkEx-based perpetuals |

### Derive (Options DEX)

| Feature | Value |
|---------|-------|
| **Referral Code** | `0x0ad42b8e602c2d3d475ae52d678cf63d84ab2749` |
| **Type** | On-chain referral address |

---

### Broker Code Injection Methods

CCXT uses **4 different methods** to inject broker codes:

#### 1. HTTP Headers (Stealth)
```python
# Bybit - Referer header
headers['Referer'] = brokerId  # 'CCXT'

# BitMart - Custom header
headers['X-BM-BROKER-ID'] = brokerId  # 'CCXTxBitmart000'

# BingX - Custom header  
headers['X-SOURCE-KEY'] = broker  # 'CCXT'

# HashKey - Custom header
headers['INPUT-SOURCE'] = broker  # '10000700011'

# Paradex - Custom header
headers['PARADEX-PARTNER'] = broker  # 'CCXT'
```

#### 2. Client Order ID Prefix
```python
# Binance - Prepends to order ID
request['newClientOrderId'] = brokerId + self.uuid22()  # 'x-TKT5PX2F' + uuid

# OKX - Prepends to client order ID
request['clOrdId'] = brokerId + self.uuid16()  # '6b9ad766b55dBCDE' + uuid
```

#### 3. Request Parameters
```python
# OKX - Tag parameter
request['tag'] = brokerId  # '6b9ad766b55dBCDE'

# MEXC - Source parameter
request['source'] = broker  # 'CCXT'

# Crypto.com - Broker ID parameter
request['params']['broker_id'] = broker  # 'CCXT'
```

#### 4. Partner Keys (KuCoin)
```python
# KuCoin uses partner ID + secret key combination
headers['KC-API-PARTNER'] = partner_id  # 'ccxt'
headers['KC-API-PARTNER-SIGN'] = hmac_signature  # Uses key: '9e58cc35-5b5e-4133-92ec-166e3f077cb8'
```

### Confirmed Exchange Partnerships

These exchanges publicly list CCXT as a partner on their broker program pages:

| Exchange | Broker Page | CCXT Listed |
|----------|-------------|-------------|
| **CoinEx** | https://www.coinex.com/broker | Yes - "Our Partners" section |
| **BitMart** | https://www.bitmart.com/broker | Yes - "CCXT PRO" listed |
| **Gate.io** | https://www.gate.io/broker | Yes - Listed as partner |

**Anomaly:** Gate.io lists CCXT as a partner, but CCXT's source code has no broker ID for Gate.io. This suggests either:
- An old partnership that was removed from code
- A referral-only partnership (no API broker code)
- Partnership pending implementation

### Exchange Broker Program URLs

| Exchange | Broker Program URL | Max Commission |
|----------|-------------------|----------------|
| Binance | https://www.binance.com/en/broker | Up to 50% |
| Bybit | https://www.bybit.com/en/partners/broker | Up to 50% |
| OKX | https://www.okx.com/broker | Up to 50% |
| HTX | https://www.htx.com/en-us/broker | Up to 65% |
| KuCoin | https://www.kucoin.com/broker | Up to 70% |
| BitMart | https://www.bitmart.com/broker | 40-50% |
| CoinEx | https://www.coinex.com/broker | 40% |
| Gate.io | https://www.gate.io/broker | Up to 70% |
| MEXC | https://www.mexc.com/broker | Varies |
| Bitget | https://www.bitget.com/broker | Varies |
| Crypto.com | https://crypto.com/exchange/broker | Varies |
| BingX | https://bingx.com/en/broker | Varies |

---

## Who Can Become a Broker?

### Typical Applicants
- Exchange aggregators
- Trading bot platforms (like Freqtrade, Hummingbot)
- Copy trading platforms
- Asset management platforms
- Quantitative trading libraries (like CCXT)
- White-label exchange solutions

### Requirements
1. Operate a legitimate trading platform/service
2. API integration capability
3. Apply via exchange's broker program
4. Sign legal agreement
5. Integrate broker code into API calls

### OKX Broker Program Example

Two models available:

**API Broker:**
- User creates OKX account → generates API Key → binds with broker
- Broker places orders with broker code + user's API Key

**OAuth Broker:**
- One-touch login via OAuth
- User authorizes broker to trade on their behalf
- More seamless (user doesn't share API key)

Orders must include broker code:
```json
{
  "instId": "BTC-USDT",
  "tdMode": "cash",
  "side": "buy",
  "ordType": "limit",
  "px": "1000",
  "sz": "0.01",
  "tag": "BROKER_CODE"  // Broker code here
}
```

---

## Exclusions from Broker Commissions

Brokers typically don't earn on:
- VIP 7+ users (high volume traders with special rates)
- Market makers with custom fee agreements
- Users with commission discount cards
- Managed trading sub-accounts

---

## Corporate Sponsors

Notable sponsors via Open Collective:

| Sponsor          | Amount  |
| ---------------- | ------- |
| TabTrader BV     | $5,500  |
| Remitano         | $2,650  |
| Stronghold       | $2,600  |
| Nomics Crypto API| $2,000  |
| COINCUBE         | $2,000  |

---

## Who Maintains CCXT?

- **Igor Kroitor** (@kroitor) - Lead developer/Admin
- **Carlo Revelli** - Admin
- **730+ contributors** on GitHub

---

## Why This Model Works

The broker model creates a **win-win-win**:

```
Users         → Get free, high-quality library
Exchanges     → Get trading volume via CCXT's 40k+ star user base  
CCXT          → Gets 20-70% of trading fees on every trade
```

### The Math

If CCXT users generate $1B/month in trading volume:
- Average trading fee: 0.1% = $1M in fees
- Broker commission (50%): **$500K/month** to CCXT

This is why they could make CCXT Pro free - the **broker commissions dwarf subscription revenue**.

---

## Competitor Broker Code Analysis

**Research Date:** January 2026

All major open-source crypto trading platforms use broker codes to monetize. Here's a comparison:

### Broker Code Comparison Table

| Platform | Uses CCXT? | Own Broker Codes? | Exchanges with Codes | Business Model |
|----------|------------|-------------------|---------------------|----------------|
| **CCXT** | N/A | Yes (28) | Binance, Bybit, OKX, KuCoin, HTX, + 23 more | Pure broker revenue |
| **Hummingbot** | No | Yes (8) | Binance, Bybit, OKX, Gate.io (spot+perp) | Non-profit + broker revenue |
| **NautilusTrader** | No | Yes (2) | Bybit (`Qy000878`), OKX (`a535cbe8d0c8BCDE`) | Open-core + broker revenue |
| **Freqtrade** | Yes | Yes (1) | OKX only (`ffb5405ad327SUDE`) | Volunteer + broker revenue |
| **Jesse** | No | Hidden | Server-side redirects (jesse.trade/bybit) | Freemium SaaS + affiliates |

### Detailed Competitor Analysis

#### CCXT (Library)
- **28 exchanges** with broker codes embedded
- Primary revenue source - funded Pro version going free
- Most comprehensive exchange coverage
- All users automatically generate commission for CCXT

#### Hummingbot (Trading Bot)
- **Non-profit foundation** model (Cayman Islands)
- Transparent about broker codes - documented on website
- Users get fee rebates (10-20% off trading fees)
- Broker codes in: Binance, Bybit, OKX, Gate.io (both spot and perpetual)
- Example: Binance Spot = `x-MG43PCSN`, OKX = `93027a12dac34fBC`

#### NautilusTrader (Institutional Framework)
- **High-performance Rust-based** platform
- Native exchange implementations (no CCXT dependency)
- Only 2 broker codes: Bybit and OKX
- Open-core model with cloud platform
- 17k GitHub stars, institutional focus

#### Freqtrade (Trading Bot)
- **Community-driven** open-source project
- Uses CCXT for most exchanges
- Only 1 custom broker code: OKX (`ffb5405ad327SUDE`)
- Most exchanges → CCXT gets the commission
- Referral links in README: BingX, Gate.io

#### Jesse (Trading Bot)
- **Freemium SaaS** model
- Closed-source live trading plugin (requires paid license)
- Broker codes hidden server-side via redirects (jesse.trade/bybit)
- Smarter approach - codes not visible in open source
- Revenue: Plugin licenses + affiliate commissions

### Key Insights

1. **Everyone uses broker codes** - It's the standard business model for free trading tools
2. **CCXT gets most commissions** - If you use CCXT, they get commission on all trades
3. **Direct partnerships pay more** - Own broker codes = full commission (not shared with CCXT)
4. **Hidden codes are smarter** - Jesse's server-side approach prevents competitors from seeing codes
5. **Non-profit model works** - Hummingbot proves broker revenue can fund sustainable development

---

## Implications for ZenBT

### Current State
- ZenBT is a **backtesting-only** framework (no live trading)
- Built with Rust (high performance) and Python bindings
- Similar architecture to NautilusTrader

### Monetization Options

If ZenBT adds live trading capabilities:

| Approach | Effort | Revenue | Notes |
|----------|--------|---------|-------|
| **Use CCXT as-is** | Low | $0 | CCXT gets all commissions |
| **CCXT + override broker IDs** | Medium | Medium | Replace CCXT's codes with own |
| **Native exchange implementations** | High | High | Full commission, like NautilusTrader |
| **Jesse model (server-side)** | Medium | High | Hidden codes, freemium plugin |
| **Cloud/SaaS platform** | High | High | Subscription + broker revenue |

### Recommended Strategy

1. **Phase 1: Use CCXT for rapid development**
   - Get live trading working quickly
   - Test with small volume
   - Learn exchange APIs

2. **Phase 2: Apply for broker programs**
   - Start with high-volume exchanges: Binance, Bybit, OKX
   - Apply through their broker program pages
   - Expected approval time: 1-4 weeks

3. **Phase 3: Override CCXT broker codes**
   - Configure CCXT with own broker IDs
   - Example: `exchange.options['brokerId'] = 'YOUR_CODE'`
   - Requires testing each exchange

4. **Phase 4: Native implementations (optional)**
   - For highest-volume exchanges
   - Full control over commission
   - Higher development cost

### Broker Program Application Checklist

For ZenBT to apply to broker programs:

- [ ] Working trading platform/bot
- [ ] Legal entity (company registration)
- [ ] Website with documentation
- [ ] Privacy policy / Terms of service
- [ ] Technical integration plan
- [ ] Expected monthly trading volume estimate

### Priority Exchange Partnerships

Based on commission rates and user demand:

| Priority | Exchange | Max Commission | Effort | Broker URL |
|----------|----------|---------------|--------|------------|
| 1 | KuCoin | 70% | Medium | kucoin.com/broker |
| 2 | HTX | 65% | Medium | htx.com/en-us/broker |
| 3 | Binance | 50% | High | binance.com/en/broker |
| 4 | OKX | 50% | Medium | okx.com/broker |
| 5 | Bybit | 50% | Low | bybit.com/en/partners/broker |
| 6 | Gate.io | 70% | Medium | gate.io/broker |

---

## Links

- GitHub: https://github.com/ccxt/ccxt
- Open Collective: https://opencollective.com/ccxt
- Documentation: https://docs.ccxt.com/
