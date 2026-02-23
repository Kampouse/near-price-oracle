# NEAR Price Oracle Contract

Decentralized price oracle for NEAR that aggregates prices from multiple sources.

**Contract:** `oracle.gorked.testnet` on NEAR testnet

## Features

- **Multi-source aggregation**: Collect prices from 3+ sources (CoinGecko, Binance, CoinMarketCap, etc.)
- **Average pricing**: Returns the average price across all sources
- **Price history**: Tracks timestamp and reporter for each price
- **Configurable**: Owner can set minimum sources required for valid price

## Contract Methods

### View Methods

```bash
# Get aggregated price (average of all sources)
near view oracle.gorked.testnet get_price --networkId testnet
# Returns: 5276666 (micro-dollars, divide by 1,000,000 for USD)

# Get number of price sources
near view oracle.gorked.testnet get_source_count --networkId testnet
# Returns: 3

# Get detailed price info from all sources
near view oracle.gorked.testnet get_price_details --networkId testnet

# Check if oracle has enough sources (valid)
near view oracle.gorked.testnet is_valid --networkId testnet

# Get last update timestamp
near view oracle.gorked.testnet get_last_update --networkId testnet
```

### Call Methods

```bash
# Report a price from an external source
# price_usd is in micro-dollars (e.g., $5.25 = 5250000)
near call oracle.gorked.testnet report_price \
  '{"source":"coingecko","price_usd":5250000}' \
  --accountId YOUR_ACCOUNT.testnet \
  --networkId testnet

# Set minimum sources required (owner only)
near call oracle.gorked.testnet set_min_sources \
  '{"min_sources":3}' \
  --accountId gorked.testnet \
  --networkId testnet

# Clear all prices (owner only)
near call oracle.gorked.testnet clear_prices \
  --accountId gorked.testnet \
  --networkId testnet
```

## Price Format

Prices are stored in **micro-dollars** (1 USD = 1,000,000 micro-dollars):

- $5.25 → `5250000`
- $10.00 → `10000000`
- $0.50 → `500000`

This avoids floating-point precision issues on-chain.

## Example Usage

```bash
# Report prices from 3 sources
near call oracle.gorked.testnet report_price '{"source":"coingecko","price_usd":5250000}' --accountId gorked.testnet --networkId testnet
near call oracle.gorked.testnet report_price '{"source":"binance","price_usd":5300000}' --accountId gorked.testnet --networkId testnet
near call oracle.gorked.testnet report_price '{"source":"coinmarketcap","price_usd":5280000}' --accountId gorked.testnet --networkId testnet

# Get the average price
near view oracle.gorked.testnet get_price --networkId testnet
# Returns: 5276666 ($5.28 average)
```

## Building

```bash
# Install cargo-near
cargo install cargo-near

# Build contract
cargo near build non-reproducible-wasm
```

## Deployment

```bash
# Create account
near create-account oracle.YOUR_ACCOUNT.testnet \
  --masterAccount YOUR_ACCOUNT.testnet \
  --initialBalance 5 \
  --networkId testnet

# Deploy contract
near deploy oracle.YOUR_ACCOUNT.testnet target/near/price_oracle.wasm --networkId testnet
```

## Technical Details

- **Built with**: near-sdk 5.17.0, cargo-near
- **Rust version**: 1.86.0 (required for NEAR compatibility)
- **Price sources**: CoinGecko, Binance, CoinMarketCap, or any other API
- **Aggregation method**: Simple average of all reported prices

## Agent Wars Challenge

This contract was built for the **Oracle Challenge** on Agent Market:
- Prize: 1000 NEAR
- Requirements: Fetch NEAR price from 3 APIs and aggregate
- Contract: oracle.gorked.testnet

## License

MIT
