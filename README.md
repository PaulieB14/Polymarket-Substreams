# Polymarket Substreams

A high-performance Substreams package for extracting and processing events from Polymarket contracts on Polygon. This package provides real-time data streaming for prediction market activities, trading events, and market maker operations.

## Overview

Polymarket is a decentralized prediction market platform built on Ethereum and Polygon. This Substreams package indexes key events from Polymarket's smart contracts to provide:

- **Trading Events**: Order fills, matches, and cancellations
- **Market Data**: Condition preparation and resolution
- **Liquidity Events**: Market maker funding and removal
- **User Activity**: Trading patterns and volume metrics

## Contracts Indexed

### CTF Exchange Contract
- **Address**: `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e`
- **Events**: OrderFilled, OrdersMatched, OrderCancelled, TokenRegistered

### Conditional Tokens Framework
- **Address**: `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045`
- **Events**: ConditionPreparation, ConditionResolution, PositionSplit, PositionMerge, PositionRedeem

### USDC Collateral Token
- **Address**: `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174`
- **Events**: Transfer, Approval

## Modules

### `map_ctf_exchange_events`
Extracts and processes events from the CTF Exchange contract, including:
- Order fills with maker/taker information
- Order matches with asset details
- Fee charging events
- Token registration events

### `map_ctf_events`
Processes Conditional Tokens Framework events:
- Condition preparation and resolution
- Position splits, merges, and redemptions
- Oracle and question ID tracking

### `map_usdc_events`
Tracks USDC token transfers and approvals for collateral management.

### `map_trading_data`
Combines data from all modules to create enriched trading information:
- Trade events with calculated metrics
- Market data aggregation
- User activity tracking
- Volume and fee calculations

## Quick Start

### Prerequisites

1. Install the [Substreams CLI](https://docs.substreams.dev/how-to-guides/install-substreams-cli)
2. Get authentication token from [The Graph Market](https://thegraph.market)

### Installation

1. Clone this repository:
```bash
git clone https://github.com/Polymarket/polymarket-substreams.git
cd polymarket-substreams
```

2. Build the Substreams:
```bash
substreams build
```

3. Authenticate with The Graph:
```bash
substreams auth
```

4. Run the Substreams:
```bash
# Run CTF Exchange events
substreams gui substreams.yaml map_ctf_exchange_events

# Run trading data aggregation
substreams gui substreams.yaml map_trading_data
```

## Usage Examples

### Extract Order Fills
```bash
substreams run -e polygon.streamingfast.io:443 substreams.yaml map_ctf_exchange_events --start-block 50000000 --stop-block 50000100
```

### Monitor Trading Activity
```bash
substreams gui substreams.yaml map_trading_data --start-block -1000
```

## Data Schema

### Trade Event
```protobuf
message Trade {
    string transaction_hash = 1;
    uint32 log_index = 2;
    google.protobuf.Timestamp timestamp = 3;
    uint64 block_number = 4;
    string trader = 5;
    string market_id = 6;
    string outcome_token = 7;
    string amount = 8;
    string price = 9;
    string fee = 10;
    string trade_type = 11; // "buy" or "sell"
}
```

### Market Event
```protobuf
message Market {
    string market_id = 1;
    string condition_id = 2;
    string question_id = 3;
    string oracle = 4;
    uint32 outcome_slot_count = 5;
    google.protobuf.Timestamp creation_timestamp = 6;
    string status = 7; // "open", "closed", "resolved"
}
```

## Performance Benefits

This Substreams implementation provides significant performance improvements over traditional subgraphs:

- **100x Faster Syncing**: Parallel processing of blockchain data
- **Real-time Streaming**: Continuous data flow instead of batch processing
- **Efficient Resource Usage**: Optimized memory and CPU utilization
- **Scalable Architecture**: Handles high-volume trading activity

## Development

### Adding New Contracts

To add support for additional Polymarket contracts:

1. Add contract address to `src/lib.rs`
2. Update `substreams.yaml` with new module
3. Add corresponding protobuf messages
4. Implement event extraction logic

### Custom Transformations

The `map_trading_data` module can be extended to include:
- Price calculations
- Volume aggregations
- User behavior analysis
- Market sentiment metrics

## Publishing

To publish this Substreams package:

1. Package the Substreams:
```bash
substreams pack
```

2. Publish to the registry:
```bash
substreams registry login
substreams registry publish polymarket-v0.1.0.spkg
```

## Contributing

Contributions are welcome! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- Documentation: [Substreams Docs](https://docs.substreams.dev)
- Community: [The Graph Discord](https://discord.gg/thegraph)
- Issues: [GitHub Issues](https://github.com/Polymarket/polymarket-substreams/issues)

## Acknowledgments

- Built with [Substreams](https://substreams.dev) by StreamingFast
- Powered by [The Graph](https://thegraph.com)
- Data from [Polymarket](https://polymarket.com)