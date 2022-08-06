use std::str::FromStr;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClient::new_http_client("https://gateway.devnet.sui.io:443")?;
    let address = SuiAddress::from_str("0x7f8e8f419d39ef61ca63b5d474084d1cd8b4e4f0")?;
    let objects = sui.get_objects_owned_by_address(address).await?;
    for sui_object in objects {
        println!("{:?}", sui_object);
    }
    // println!("{:?}", objects);
    Ok(())
}
