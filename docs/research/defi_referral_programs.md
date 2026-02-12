# DeFi Referral & Builder Programs

*Last updated: January 2026*

## Overview

DeFi protocols have different monetization mechanisms than CeFi exchanges. This document analyzes how referral/affiliate programs work in DeFi, specifically for protocols connected by CCXT.

---

## Key Differences: CeFi vs DeFi

| Aspect               | CeFi Broker Programs           | DeFi Builder/Referral                  |
| -------------------- | ------------------------------ | -------------------------------------- |
| **Commission Model**     | Revenue share (20-70% of fees) | Fee markup or revenue share            |
| **Typical Rates**        | 20-50% of trading fees         | 0.01% - 1% of trade value              |
| **Implementation**       | API headers (X-Broker-ID)      | On-chain parameters or smart contracts |
| **User Consent**         | Implicit (via library)         | Often explicit (user signs approval)   |
| **Settlement**           | Off-chain, periodic            | On-chain, per-trade or claimable       |
| **Transparency**         | Opaque                         | Fully transparent on-chain             |
| **KYC Requirement**      | Usually required for broker    | Usually not required                   |

---

## DEXes in CCXT with Referral/Builder Programs

### Summary Table

| DEX | Type | Has Broker/Builder Code | CCXT Code | Commission |
|-----|------|------------------------|-----------|------------|
| **Hyperliquid** | Perp DEX | Yes (Builder) | `0x6530512A...` | Up to 0.1% |
| **Apex** | Perp DEX | Yes (Broker) | `6956` | Unknown |
| **WOOFi Pro** | DEX | Yes (Broker) | `CCXT` | Unknown |
| **ModeTrade** | DEX | Yes (Broker) | `CCXTMODE` | Unknown |
| **Derive** | Options DEX | Yes (Referral) | `0x0ad42b8e...` | Unknown |
| **Paradex** | Perp DEX | Yes (Partner) | `CCXT` | Unknown |
| **dYdX** | Perp DEX | Has builder codes | None in CCXT | ~10-20% |
| **Defx** | DEX | Referral API exists | None | Unknown |
| **Hibachi** | DEX | Unknown | None | Unknown |
| **Waves** | DEX | No | None | N/A |

---

## Detailed Analysis

### 1. Hyperliquid - Builder Codes

**Most sophisticated DeFi referral system**

Hyperliquid has TWO separate programs:

#### Builder Codes (B2B - for integrators)

| Feature | Details |
|---------|---------|
| **Purpose** | Allow third-party apps to earn fees on trades |
| **Max Fee (Perps)** | 0.1% (10 basis points) |
| **Max Fee (Spot)** | 1% (100 basis points) |
| **User Consent** | Required - user must sign `ApproveBuilderFee` |
| **Minimum Requirement** | 100 USDC in perps account |

**CCXT Implementation:**
```python
# CCXT's builder address
builder = '0x6530512A6c89C7cfCEbC3BA7fcD9aDa5f30827a6'

# Order includes builder info
orderAction['builder'] = {
    'b': wallet,  # Builder address
    'f': 10       # Fee in tenths of bps (10 = 1 bps = 0.01%)
}
```

#### Referral Codes (B2C - for individuals)

| Feature | Details |
|---------|---------|
| **Referrer Reward** | 10% of referred users' fees |
| **Referee Discount** | 4% fee discount |
| **Volume to Create Code** | $10,000 trading volume |
| **Referrer Cap** | First $1B in volume |
| **Referee Cap** | First $25M in volume |

**CCXT Implementation:**
```python
# CCXT sets referral code
action = {
    'type': 'setReferrer',
    'code': 'CCXT1'  # Referral code
}
```

---

### 2. Apex Pro

**Broker ID system similar to CeFi**

| Feature | Details |
|---------|---------|
| **Broker ID** | `6956` |
| **Implementation** | Sent in order request |

**CCXT Implementation:**
```python
'brokerId': self.safe_string(self.options, 'brokerId', '6956')
```

---

### 3. WOOFi Pro (Orderly Network)

**Built on Orderly Network infrastructure**

| Feature | Details |
|---------|---------|
| **Broker ID** | `CCXT` |
| **Key Broker ID** | `woofi_pro` |
| **Network** | Orderly Network (omnichain) |

**CCXT Implementation:**
```python
'brokerId': 'CCXT'
'keyBrokerId': 'woofi_pro'
```

---

### 4. ModeTrade

**Similar to WOOFi Pro, also on Orderly Network**

| Feature | Details |
|---------|---------|
| **Broker ID** | `CCXTMODE` |
| **Key Broker ID** | `mode` |

---

### 5. Derive (formerly Lyra)

**Options DEX with on-chain referral**

| Feature | Details |
|---------|---------|
| **Referral Code** | Ethereum address |
| **CCXT Code** | `0x0ad42b8e602c2d3d475ae52d678cf63d84ab2749` |
| **Type** | On-chain referral tracking |

---

### 6. Paradex

**StarkEx-based perpetuals DEX**

| Feature | Details |
|---------|---------|
| **Partner Header** | `PARADEX-PARTNER` |
| **CCXT Code** | `CCXT` |
| **Has Referral API** | Yes (`/referrals/*` endpoints) |

---

### 7. dYdX v4

**Cosmos-based perpetuals DEX**

| Feature | Details |
|---------|---------|
| **Referral System** | On-chain affiliate program |
| **Commission** | ~10-20% of referred fees (tiered) |
| **CCXT Status** | No builder code embedded |

dYdX has builder codes but CCXT hasn't implemented them yet.

---

## DEX Aggregator Referral Programs (Not in CCXT)

### Jupiter (Solana)

| Feature | Details |
|---------|---------|
| **Protocol Fee** | 0% by default |
| **Integrator Fee** | `platformFeeBps` parameter (flexible) |
| **Implementation** | Pass `feeAccount` in swap request |

```javascript
// Example: 0.2% integrator fee
fetch('https://api.jup.ag/swap/v1/quote?' +
  'platformFeeBps=20&feeAccount=YOUR_TOKEN_ACCOUNT'
)
```

### 0x Protocol

| Feature | Details |
|---------|---------|
| **Affiliate Fee** | Up to 10% of trade value |
| **Trade Surplus** | Capture positive slippage (custom plans) |
| **Parameters** | `swapFeeRecipient`, `swapFeeBps`, `swapFeeToken` |

### 1inch

| Feature | Details |
|---------|---------|
| **Integrator Program** | Revenue sharing for API integrators |
| **Fusion Mode** | Different fee structures |

---

## Commission Rate Comparison

| Platform | Type | Rate Structure |
|----------|------|----------------|
| **Binance Broker** | CeFi | Up to 50% of fees |
| **Bybit Broker** | CeFi | Up to 50% of fees |
| **OKX Broker** | CeFi | Up to 50% of fees |
| **Hyperliquid Builder** | DeFi | Up to 0.1% of trade value |
| **Hyperliquid Referral** | DeFi | 10% of referred fees |
| **Jupiter** | DeFi | Integrator-defined |
| **0x** | DeFi | Up to 10% of trade value |
| **dYdX** | DeFi | ~10-20% of fees (tiered) |

**Key Insight:** CeFi programs offer higher revenue share (% of fees), while DeFi programs offer lower rates but are often additive fees on top of trades.

---

## Technical Implementation Differences

### CeFi (Example: Binance)
```python
# Fee attribution via HTTP header - invisible to user
headers = {
    "X-MBX-APIKEY": api_key,
}
# Broker ID embedded in client order ID
order_id = "x-TKT5PX2F" + uuid  # Broker prefix
```

### DeFi (Example: Hyperliquid)
```python
# Fee attribution via on-chain parameter
# User MUST approve builder fee first!
order_action = {
    "type": "order",
    "orders": [{...}],
    "builder": {
        "b": "0xBuilderAddress",
        "f": 10  # Fee in tenths of bps
    }
}
```

### DeFi (Example: Jupiter)
```javascript
// Fee via API + on-chain token account
const quote = await fetch(
  `https://api.jup.ag/swap/v1/quote?platformFeeBps=20`
);
// User pays fee directly, goes to feeAccount
```

---

## Implications for ZenBT

### If Adding DeFi Support

| Approach | Effort | Revenue Potential |
|----------|--------|-------------------|
| **Use CCXT DEX connectors** | Low | CCXT gets builder fees |
| **Apply for Hyperliquid Builder** | Medium | Up to 0.1% per trade |
| **Build Jupiter integration** | Medium | Flexible platform fees |
| **dYdX affiliate program** | Low | 10-20% of fees |

### Priority DeFi Partnerships

| Priority | Protocol | Why |
|----------|----------|-----|
| 1 | **Hyperliquid** | Largest perp DEX, builder program |
| 2 | **Jupiter** | Largest Solana aggregator, flexible fees |
| 3 | **dYdX** | Established perp DEX, affiliate program |
| 4 | **0x** | Multi-chain aggregator, easy integration |

### Key Considerations

1. **User consent required** - Unlike CeFi, DeFi builder fees often require explicit user approval
2. **On-chain transparency** - All fees are visible on-chain
3. **Lower rates** - DeFi typically offers lower commission rates than CeFi
4. **Non-custodial** - No need for API broker agreements, just smart contract integration

---

## Links

### DEX Documentation
- Hyperliquid Builder: https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes
- Hyperliquid Referrals: https://hyperliquid.gitbook.io/hyperliquid-docs/referrals
- Jupiter Referral: https://dev.jup.ag/tool-kits/referral-program
- 0x Monetization: https://docs.0x.org/0x-swap-api/guides/monetize-your-app-using-swap
- dYdX: https://dydx.exchange/

### CCXT DEX Connectors
- Hyperliquid: https://github.com/ccxt/ccxt/blob/master/python/ccxt/hyperliquid.py
- Apex: https://github.com/ccxt/ccxt/blob/master/python/ccxt/apex.py
- WOOFi Pro: https://github.com/ccxt/ccxt/blob/master/python/ccxt/woofipro.py
- dYdX: https://github.com/ccxt/ccxt/blob/master/python/ccxt/dydx.py
