use sui::client_commands::WalletContext;
use sui_config::{sui_config_dir, SUI_CLIENT_CONFIG};
use sui_sdk::types::base_types::SuiAddress;

/// Get the total balance of gas for Sui address
async fn get_total_gas_balance(
    wallet: &WalletContext,
    address: SuiAddress,
) -> Result<u64, anyhow::Error> {
    let balances = wallet.gas_objects(address).await?;
    let mut total_balance = 0u64;
    for gas in balances {
        total_balance = total_balance + gas.0;
    }
    Ok(total_balance)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Instantiate wallet context
    let mut config_path = sui_config_dir()?;
    config_path.push(SUI_CLIENT_CONFIG);
    let wallet = WalletContext::new(&config_path).await?;

    // Get total gas balance
    println!(
        "Total balance {}",
        get_total_gas_balance(&wallet, wallet.config.active_address.unwrap()).await?
    );
    Ok(())
}
