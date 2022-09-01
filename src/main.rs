use sui::client_commands::WalletContext;
use sui_config::{sui_config_dir, SUI_CLIENT_CONFIG};
use sui_sdk::types::base_types::SuiAddress;

/// Get the total balance of gas for Sui address
async fn get_total_gas_balance(
    wallet: &WalletContext,
    address: &SuiAddress,
) -> Result<u64, anyhow::Error> {
    let balances = wallet.gas_objects(*address).await?;
    let mut total_balance = 0u64;
    for gas in balances {
        let v1 = gas.1;
        println!("  {} = {}", v1.id(), gas.0);
        total_balance = total_balance + gas.0;
    }
    Ok(total_balance)
}

/// Get and parse contract objects for address
/// Probably a better way but... still putting this together
async fn get_owned_contracts(
    wallet: &WalletContext,
    address: &SuiAddress,
) -> Result<u8, anyhow::Error> {
    let mut cnt = 0u8;
    for object in wallet
        .gateway
        .read_api()
        .get_objects_owned_by_address(*address)
        .await?
    {
        let obj_type = object.type_.split("::").collect::<Vec<&str>>();
        if obj_type[1] != "coin" {
            println!("Address {} = {}", obj_type[0], obj_type[2]);
            cnt = cnt + 1;
        }
    }
    Ok(cnt)
}

async fn inspect_walet(wallet: &WalletContext) -> Result<(), anyhow::Error> {
    // Show for each adddress
    for add in wallet.keystore.addresses() {
        println!("\nWallet address  {}", add);
        println!("Gas");
        println!("---");
        // Get total gas balance
        println!(
            "   Total balance = {}",
            get_total_gas_balance(&wallet, &add).await?
        );
        println!("Contracts");
        println!("--------");
        if get_owned_contracts(&wallet, &add).await? == 0 {
            println!("None")
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Instantiate wallet context
    let mut config_path = sui_config_dir()?;
    config_path.push(SUI_CLIENT_CONFIG);
    let wallet = WalletContext::new(&config_path).await?;
    inspect_walet(&wallet).await?;

    Ok(())
}
