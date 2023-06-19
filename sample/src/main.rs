use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::{SuiClientBuilder};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default().build(
        "https://fullnode.devnet.sui.io:443",
    ).await.unwrap();
    let rgp = sui.read_api().get_reference_gas_price().await?;
    println!("{:?}", rgp);
    Ok(())
}