# Trading Execution Systems - Broker Program Participation Research

## Executive Summary

| System         | Broker Codes       | Disclosed | Primary Revenue                 |
| -------------- | ------------------ | --------- | ------------------------------- |
| **NautilusTrader** | Yes (Bybit, OKX)   | Partial   | Open source + Broker fees       |
| **Freqtrade**      | Yes (OKX)          | No        | Community + Broker fees         |
| **Jesse**          | Unknown            | No        | Paid live trading plugin        |
| **Hummingbot**     | Yes (5 exchanges, 9 connectors) | Yes       | Broker fees + Protocol sponsors |
| **OctoBot**        | No                 | N/A       | SaaS subscriptions              |

---

## 1. NautilusTrader

**Broker codes found:** Yes

| Exchange | Broker ID | Injection Method    | File Path                                  |
| -------- | --------- | ------------------- | ------------------------------------------ |
| Bybit    | `Qy000878`  | `Referer` HTTP header | `crates/adapters/bybit/src/common/consts.rs` |
| OKX      | `a535cbe8d0c8BCDE` | Client Order ID / Tag | `crates/adapters/okx/src/common/consts.rs` |
| Binance  | None      | -                   | -                                          |

**Code snippet:**
```rust
/// See <https://www.bybit.com/en/broker> for further details.
pub const BYBIT_NAUTILUS_BROKER_ID: &str = "Qy000878";
```

**Public disclosure:** Partial - only in code comments, not in README/docs

**Business model:** Open source (LGPL-3.0), commercial support from Nautech Systems Pty Ltd (Australia)

---

## 2. Freqtrade

**Broker codes found:** Yes

| Exchange | Broker ID        | Injection Method     | File Path                 |
| -------- | ---------------- | -------------------- | ------------------------- |
| OKX      | `ffb5405ad327SUDE` | CCXT `brokerId` option | `freqtrade/exchange/okx.py` |
| Binance  | None             | -                    | -                         |
| Bybit    | None             | -                    | -                         |

**Code snippet:**
```python
class Okx(Exchange):
    _ccxt_params: dict = {"options": {"brokerId": "ffb5405ad327SUDE"}}
```

**Also found:** Referral links in docs for BingX (`0EM9RX`) and Gate.io (`6266643`)

**Public disclosure:** No - not disclosed in README, docs, or CONTRIBUTING

**Business model:** Open source community-driven, GitHub Sponsors, undisclosed broker commissions

---

## 3. Jesse

**Broker codes found:** Cannot verify - live trading is closed source

| Component                  | Broker Codes | Notes                        |
| -------------------------- | ------------ | ---------------------------- |
| Open source (backtesting)  | None found   | Clean candle import drivers  |
| Paid plugin (live trading) | Unknown      | Closed source, cannot verify |

**Public disclosure:** No mention of exchange partnerships

**Business model:** Freemium model - free backtesting, paid live trading subscription

---

## 4. Hummingbot

**Broker codes found:** Yes - most comprehensive participation

| Exchange     | Broker ID                | Injection Method       | User Rebate | Source File |
| ------------ | ------------------------ | ---------------------- | ----------- | ----------- |
| Binance Spot | `x-MG43PCSN`               | Client order ID prefix | 10%         | `binance_constants.py` |
| Binance Perp | `x-nbQe1H39`               | Client order ID prefix | 10%         | `binance_perpetual_constants.py` |
| OKX Spot     | `93027a12dac34fBC`         | Client ID prefix       | 20%         | `okx_constants.py` |
| OKX Perp     | `93027a12dac34fBC`         | Client ID prefix       | 20%         | `okx_perpetual_constants.py` |
| Bybit Spot   | `Hummingbot`               | Referer header         | -           | `bybit_constants.py` |
| Bybit Perp   | `Hummingbot`               | Referer header         | -           | `bybit_perpetual_constants.py` |
| Gate.io Spot | `hummingbot`               | Broker ID header       | 20%         | `gate_io_constants.py` |
| Gate.io Perp | `hummingbot`               | Broker ID header       | 20%         | `gate_io_perpetual_constants.py` |
| HTX          | `AAc484720a`               | Broker ID              | 20%         | `htx_constants.py` |

**Public disclosure:** Yes - fully transparent at [hummingbot.org/about/sponsors/](https://hummingbot.org/about/sponsors/)

> "Our exchange partners share a portion of user-generated fees with the Foundation, at zero cost to users."

**Business model:** Not-for-profit Foundation (Cayman Islands), broker fee-sharing, protocol sponsorships, HBOT governance token

---

## 5. OctoBot

**Broker codes found:** No

| Component         | Broker Codes | Notes                      |
| ----------------- | ------------ | -------------------------- |
| CCXT connector    | None         | Clean implementation       |
| Exchange adapters | None         | No API broker IDs injected |

**Referral links in README:** Binance (`528112221`), Bybit (`QW6O5`), MEXC (`1fqGu`), Kucoin (`rJ2Q2T3`) - these are website signup links, NOT API broker codes

**Public disclosure:** Referral links visible in README, no formal disclosure

**Business model:** OctoBot Cloud SaaS subscriptions, referral links, sponsors (Chatwoot, Scaleway, Sentry)

---

## Transparency Ranking

| Rank | System         | Disclosure Level                                    |
| ---- | -------------- | --------------------------------------------------- |
| 1    | **Hummingbot**     | Full transparency - public disclosure, user rebates |
| 2    | **OctoBot**        | N/A - no broker codes, visible referral links       |
| 3    | **NautilusTrader** | Partial - only in code comments                     |
| 4    | **Freqtrade**      | None - undisclosed OKX broker participation         |
| 5    | **Jesse**          | Unknown - closed source live trading                |

---

## Key Takeaways

1. **Broker programs are common** - Most trading systems participate in at least one exchange's broker program

2. **Disclosure varies widely** - Only Hummingbot has full public disclosure; others range from partial to none

3. **User impact is minimal** - Broker fees come from exchange's share, not additional user fees

4. **CCXT makes it easy** - Most systems use CCXT's `brokerId` option for simple injection

5. **Business sustainability** - Broker programs provide sustainable funding for open source development

---

## Comparison with CCXT

For reference, CCXT (which many of these systems use) also participates in broker programs:

**CeFi Exchanges (22):**
- **Binance Spot/Margin:** `x-TKT5PX2F` (client order ID prefix)
- **Binance Futures:** `x-cvBPrNm9` (client order ID prefix)
- **OKX:** `6b9ad766b55dBCDE` (broker ID in `clOrdId` and `tag`)
- **Bybit:** `CCXT` (`Referer` HTTP header)
- **And 18 more CeFi exchanges**

**DeFi Exchanges (6):**
- **Hyperliquid:** `0x6530512A6c89C7cfCEbC3BA7fcD9aDa5f30827a6` (Builder address)
- **Apex:** `6956` (Broker ID)
- **WOOFi Pro:** `CCXT` (Broker ID on Orderly Network)
- **ModeTrade:** `CCXTMODE` (Broker ID on Orderly Network)
- **Paradex:** `CCXT` (Partner header)
- **Derive:** `0x0ad42b8e602c2d3d475ae52d678cf63d84ab2749` (Referral code)

CCXT's broker participation is disclosed in their code but not prominently in documentation.

---

## DeFi Builder/Referral Programs

DeFi protocols use different monetization mechanisms than CeFi broker programs:

### Key Differences

| Aspect | CeFi Broker | DeFi Builder |
|--------|-------------|--------------|
| **User Consent** | Implicit (hidden in library) | Often explicit (user approval required) |
| **Transparency** | Opaque | On-chain, fully visible |
| **Commission** | 20-70% of exchange fees | 0.01-1% of trade value |
| **Settlement** | Off-chain, periodic | On-chain, per-trade |

### Hyperliquid (Most Sophisticated)

| Program | Rate | Details |
|---------|------|---------|
| **Builder Code** | Up to 0.1% (perps), 1% (spot) | Requires user approval via `ApproveBuilderFee` |
| **Referral Code** | 10% of referred fees | Code: `CCXT1`, caps at $1B/$25M volume |

```python
# Builder fee in order
orderAction['builder'] = {'b': '0x6530512A...', 'f': 10}  # 0.01% fee

# Referral code (separate)
action = {'type': 'setReferrer', 'code': 'CCXT1'}
```

### DEX Aggregators (Not in CCXT)

| Aggregator | Fee Mechanism | Max Fee |
|------------|---------------|---------|
| **Jupiter** | `platformFeeBps` parameter | Flexible |
| **0x** | `swapFeeBps` parameter | Up to 10% |
| **1inch** | Integrator revenue share | Varies |

---

*Research conducted: January 2026*
