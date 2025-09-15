mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

// Contract addresses
const CTFEXCHANGE_TRACKED_CONTRACT: [u8; 20] = hex!("4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e");
const CTF_CONTRACT: [u8; 20] = hex!("4D97DCd97eC945f40cF65F87097ACe5EA0476045");
const USDC_CONTRACT: [u8; 20] = hex!("2791Bca1f2de4661ED88A30C99A7a9449Aa84174");

fn map_ctfexchange_events_old(blk: &eth::Block, events: &mut contract::CtfExchangeEvents) {
    events.ctfexchange_fee_chargeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::FeeCharged::match_and_decode(log) {
                        return Some(contract::CtfexchangeFeeCharged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            receiver: event.receiver,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_new_admins.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::NewAdmin::match_and_decode(log) {
                        return Some(contract::CtfexchangeNewAdmin {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            admin: event.admin,
                            new_admin_address: event.new_admin_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_new_operators.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::NewOperator::match_and_decode(log) {
                        return Some(contract::CtfexchangeNewOperator {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            admin: event.admin,
                            new_operator_address: event.new_operator_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_order_cancelleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::OrderCancelled::match_and_decode(log) {
                        return Some(contract::CtfexchangeOrderCancelled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            order_hash: Vec::from(event.order_hash),
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_order_filleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::OrderFilled::match_and_decode(log) {
                        return Some(contract::CtfexchangeOrderFilled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            fee: event.fee.to_string(),
                            maker: event.maker,
                            maker_amount_filled: event.maker_amount_filled.to_string(),
                            maker_asset_id: event.maker_asset_id.to_string(),
                            order_hash: Vec::from(event.order_hash),
                            taker: event.taker,
                            taker_amount_filled: event.taker_amount_filled.to_string(),
                            taker_asset_id: event.taker_asset_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_orders_matcheds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::OrdersMatched::match_and_decode(log) {
                        return Some(contract::CtfexchangeOrdersMatched {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            maker_amount_filled: event.maker_amount_filled.to_string(),
                            maker_asset_id: event.maker_asset_id.to_string(),
                            taker_amount_filled: event.taker_amount_filled.to_string(),
                            taker_asset_id: event.taker_asset_id.to_string(),
                            taker_order_hash: Vec::from(event.taker_order_hash),
                            taker_order_maker: event.taker_order_maker,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_proxy_factory_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::ProxyFactoryUpdated::match_and_decode(log) {
                        return Some(contract::CtfexchangeProxyFactoryUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_proxy_factory: event.new_proxy_factory,
                            old_proxy_factory: event.old_proxy_factory,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_removed_admins.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::RemovedAdmin::match_and_decode(log) {
                        return Some(contract::CtfexchangeRemovedAdmin {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            admin: event.admin,
                            removed_admin: event.removed_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_removed_operators.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::RemovedOperator::match_and_decode(log) {
                        return Some(contract::CtfexchangeRemovedOperator {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            admin: event.admin,
                            removed_operator: event.removed_operator,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_safe_factory_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::SafeFactoryUpdated::match_and_decode(log) {
                        return Some(contract::CtfexchangeSafeFactoryUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_safe_factory: event.new_safe_factory,
                            old_safe_factory: event.old_safe_factory,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_token_registereds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::TokenRegistered::match_and_decode(log) {
                        return Some(contract::CtfexchangeTokenRegistered {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            condition_id: Vec::from(event.condition_id),
                            token0: event.token0.to_string(),
                            token1: event.token1.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_trading_pauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::TradingPaused::match_and_decode(log) {
                        return Some(contract::CtfexchangeTradingPaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pauser: event.pauser,
                        });
                    }

                    None
                })
        })
        .collect());
    events.ctfexchange_trading_unpauseds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == CTFEXCHANGE_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::ctfexchange_contract::events::TradingUnpaused::match_and_decode(log) {
                        return Some(contract::CtfexchangeTradingUnpaused {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pauser: event.pauser,
                        });
                    }

                    None
                })
        })
        .collect());
}
// CTF Exchange Events Handler
#[substreams::handlers::map]
fn map_ctf_exchange_events(blk: eth::Block) -> Result<contract::CtfExchangeEvents, substreams::errors::Error> {
    let mut events = contract::CtfExchangeEvents::default();
    map_ctfexchange_events_old(&blk, &mut events);
    Ok(events)
}

// CTF Events Handler (placeholder - would need CTF ABI)
#[substreams::handlers::map]
fn map_ctf_events(_blk: eth::Block) -> Result<contract::CtfEvents, substreams::errors::Error> {
    let events = contract::CtfEvents::default();
    // TODO: Implement CTF event extraction when ABI is available
    Ok(events)
}

// USDC Events Handler (placeholder - would need USDC ABI)
#[substreams::handlers::map]
fn map_usdc_events(_blk: eth::Block) -> Result<contract::UsdcEvents, substreams::errors::Error> {
    let events = contract::UsdcEvents::default();
    // TODO: Implement USDC event extraction when ABI is available
    Ok(events)
}

// Combined Trading Data Handler
#[substreams::handlers::map]
fn map_trading_data(blk: eth::Block) -> Result<contract::TradingData, substreams::errors::Error> {
    let mut trading_data = contract::TradingData {
        total_trades: 0,
        total_volume: "0".to_string(),
        block_number: blk.number,
        block_timestamp: Some(blk.timestamp().to_owned()),
        ..Default::default()
    };

    // Extract trading data from CTF Exchange events
    let mut trade_count = 0;
    let mut total_volume = 0u64;

    for receipt in blk.receipts() {
        for log in &receipt.receipt.logs {
            if log.address == CTFEXCHANGE_TRACKED_CONTRACT {
                // Process OrderFilled events
                if let Some(event) = abi::ctfexchange_contract::events::OrderFilled::match_and_decode(log) {
                    let trade = contract::Trade {
                        transaction_hash: Hex(&receipt.transaction.hash).to_string(),
                        log_index: log.block_index,
                        timestamp: Some(blk.timestamp().to_owned()),
                        block_number: blk.number,
                        trader: Hex(&event.taker).to_string(),
                        market_id: event.maker_asset_id.to_string(),
                        outcome_token: event.taker_asset_id.to_string(),
                        amount: event.taker_amount_filled.to_string(),
                        price: "0".to_string(), // Would need to calculate from amounts
                        fee: event.fee.to_string(),
                        trade_type: "buy".to_string(), // Simplified
                    };
                    trading_data.trade_events.push(trade);
                    trade_count += 1;
                    
                    // Add to total volume (simplified)
                    total_volume += event.taker_amount_filled.to_u64();
                }
            }
        }
    }

    trading_data.total_trades = trade_count;
    trading_data.total_volume = total_volume.to_string();

    Ok(trading_data)
}

