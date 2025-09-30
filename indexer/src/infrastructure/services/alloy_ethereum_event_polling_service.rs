use std::error::Error;

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent,
};
use async_trait::async_trait;

use crate::domain::{
    entities::ethereum_event::EthereumEvent,
    services::ethereum_event_polling_service::EthereumEventPollingService,
};

#[derive(Clone)]
pub struct AlloyEthereumEventPollingService {}

impl AlloyEthereumEventPollingService {
    pub fn new() -> Self {
        Self {}
    }
}

sol!(ITokenActivity, "contracts/activity_token_abi.json");

#[async_trait]
impl EthereumEventPollingService for AlloyEthereumEventPollingService {
    async fn poll(&self) -> Result<Vec<EthereumEvent>, Box<dyn Error>> {
        let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse()?;
        let provider = ProviderBuilder::new().connect_http(rpc_url);

        // Get logs from the latest block
        let latest_block = provider.get_block_number().await?;

        // Create a filter to get all logs from the latest block.
        let filter = Filter::new()
            .from_block(latest_block)
            .event("Transfer(from,to,value)");

        // Get all logs from the latest block that match the filter.
        let logs = provider.get_logs(&filter).await?;

        let mut events: Vec<EthereumEvent> = Vec::<EthereumEvent>::new();

        for log in logs {
            let transfer = ITokenActivity::Transfer::decode_log_data(log.data())?;
            if transfer.from == Address::ZERO {
                let tx_hash = log.transaction_hash.unwrap();
                let log_index = log.log_index.unwrap();
                let event = EthereumEvent::new_activity_token_minted_from_transfer_data(
                    tx_hash.to_string(),
                    log_index.to_string(),
                    transfer.to,
                    transfer.value,
                );
                events.push(event);
            }
        }

        Ok(events)
    }
}
