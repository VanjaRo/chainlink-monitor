use ethers::{abi::AbiDecode, prelude::*};
use eyre::Result;
use std::collections::HashMap;
use std::{str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    // get provider Web Socket Url from .env file
    dotenv::dotenv().ok();
    let provider_ws_url = std::env::var("PROVIDER_WS_URL").unwrap();

    // TODO: receive token name pair by calling contract's describe method
    // Hardcoded contract addresses to track token pair price change
    let filtered_addreses: Vec<Address> = vec![
        Address::from_str("0x7De0d6fce0C128395C488cb4Df667cdbfb35d7DE").unwrap(),
        Address::from_str("0xe5BbBdb2Bb953371841318E1Edfbf727447CeF2E").unwrap(),
        Address::from_str("0xbba12740DE905707251525477bAD74985DeC46D2").unwrap(),
    ];

    let mut addr_to_name_map = HashMap::<Address, &str>::new();
    addr_to_name_map.insert(filtered_addreses[0], &"USDT/ETH");
    addr_to_name_map.insert(filtered_addreses[1], &"USDC/ETH");
    addr_to_name_map.insert(filtered_addreses[2], &"LINK/ETH");

    // init WS connection with a node
    let provider_ws = Provider::<Ws>::connect(provider_ws_url)
        .await
        .expect("could not instantiate Ws client");
    let provider_ws = Arc::new(provider_ws);

    // get events only from a curated list of contracts
    let filter = Filter::new()
        .address(filtered_addreses.clone())
        .event("AnswerUpdated(int256,uint256,uint256)");

    // process event stram
    let mut stream = provider_ws.subscribe_logs(&filter).await?;

    while let Some(log) = stream.next().await {
        // TODO: find more intuitve way to map log to event struct
        // log.topics[0] - abi
        // log.topics[1] - new price
        // log.topics[2] - round id
        // log.data      - update timestamp
        println!(
            "token pair: {:?}, new price: {:?}, timestamp: {:?}",
            addr_to_name_map.get(&log.address).unwrap(),
            H256::decode(log.topics[1]).unwrap().to_low_u64_be(),
            H256::decode(log.data).unwrap().to_low_u64_be(),
        );
    }

    Ok(())
}
