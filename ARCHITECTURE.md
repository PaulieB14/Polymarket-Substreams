# Polymarket Substreams Architecture

## Overview

This document describes the architecture and design decisions for the Polymarket Substreams package, built following the [Substreams development guidelines](https://docs.substreams.dev/how-to-guides/develop-your-own-substreams).

## Architecture Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Polygon       │    │   Substreams     │    │   Output        │
│   Blockchain    │───▶│   Modules        │───▶│   Data          │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌──────────────────┐
                       │   Protobuf       │
                       │   Schema         │
                       └──────────────────┘
```

## Module Architecture

### 1. CTF Exchange Events Module (`map_ctf_exchange_events`)

**Purpose**: Extract and process events from the CTF Exchange contract

**Input**: Ethereum blocks filtered for CTF Exchange contract events
**Output**: `CtfExchangeEvents` protobuf message

**Events Processed**:
- `OrderFilled`: Trading activity with maker/taker details
- `OrdersMatched`: Order matching events
- `OrderCancelled`: Order cancellation events
- `TokenRegistered`: New token registration
- `FeeCharged`: Fee collection events

### 2. CTF Events Module (`map_ctf_events`)

**Purpose**: Process Conditional Tokens Framework events

**Input**: Ethereum blocks filtered for CTF contract events
**Output**: `CtfEvents` protobuf message

**Events Processed**:
- `ConditionPreparation`: New condition creation
- `ConditionResolution`: Condition outcome resolution
- `PositionSplit`: Token position splitting
- `PositionMerge`: Token position merging
- `PositionRedeem`: Position redemption

### 3. USDC Events Module (`map_usdc_events`)

**Purpose**: Track USDC collateral token events

**Input**: Ethereum blocks filtered for USDC contract events
**Output**: `UsdcEvents` protobuf message

**Events Processed**:
- `Transfer`: USDC token transfers
- `Approval`: USDC token approvals

### 4. Trading Data Module (`map_trading_data`)

**Purpose**: Combine and enrich data from all modules

**Input**: Raw Ethereum blocks
**Output**: `TradingData` protobuf message

**Features**:
- Trade event aggregation
- Volume calculations
- User activity tracking
- Market data enrichment

## Data Flow

```
Block Data
    │
    ├── CTF Exchange Events ──┐
    ├── CTF Events ──────────┼──▶ Trading Data Module ──▶ Enriched Output
    └── USDC Events ─────────┘
```

## Contract Addresses

| Contract | Address | Purpose |
|----------|---------|---------|
| CTF Exchange | `0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e` | Order matching and trading |
| CTF | `0x4D97DCd97eC945f40cF65F87097ACe5EA0476045` | Conditional tokens |
| USDC | `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174` | Collateral token |

## Performance Optimizations

### Parallel Processing
- Each module processes events independently
- Substreams enables parallel execution across modules
- Block filtering reduces unnecessary data processing

### Memory Efficiency
- Streaming data processing instead of batch loading
- Efficient protobuf serialization
- Minimal memory footprint per block

### Network Optimization
- Block filtering at the network level
- Only relevant events are transmitted
- Compressed data transmission

## Error Handling

### Graceful Degradation
- Missing events don't break the pipeline
- Partial data is still processed
- Error logging for debugging

### Data Validation
- Protobuf schema validation
- Type safety with Rust
- Range checking for numeric values

## Scalability Considerations

### High Volume Trading
- Designed to handle Polymarket's trading volume
- Efficient event processing
- Real-time data streaming

### Future Extensions
- Modular design allows easy addition of new contracts
- Protobuf schema evolution support
- Configurable parameters

## Security Considerations

### Data Integrity
- Immutable blockchain data source
- Cryptographic verification of events
- No data modification after extraction

### Access Control
- Authentication required for live data
- Rate limiting on API calls
- Secure token handling

## Monitoring and Observability

### Metrics
- Events processed per second
- Block processing latency
- Error rates and types

### Logging
- Structured logging with timestamps
- Event-level debugging information
- Performance metrics

## Deployment

### Local Development
```bash
substreams build
substreams gui substreams.yaml map_ctf_exchange_events
```

### Production
```bash
substreams run -e polygon.streamingfast.io:443 substreams.yaml map_trading_data
```

### Publishing
```bash
substreams pack
substreams registry publish polymarket-v0.1.0.spkg
```

## Future Enhancements

### Planned Features
- Additional contract support
- Enhanced price calculations
- User behavior analytics
- Market sentiment analysis

### Performance Improvements
- Caching layer for frequently accessed data
- Batch processing optimizations
- Advanced filtering capabilities

## Dependencies

### Core Dependencies
- `substreams`: Core Substreams framework
- `substreams-ethereum`: Ethereum-specific functionality
- `prost`: Protobuf serialization
- `ethabi`: Ethereum ABI decoding

### Development Dependencies
- `hex-literal`: Hex string literals
- `num-traits`: Numeric trait implementations

## Conclusion

This architecture provides a robust, scalable foundation for processing Polymarket data using Substreams. The modular design allows for easy extension and maintenance while providing significant performance benefits over traditional indexing approaches.

For more information, see the [Substreams documentation](https://docs.substreams.dev) and [Polymarket documentation](https://docs.polymarket.com).
