use::std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Singer, TransactionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "gas monster ski craft below illegal discover limit dig bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint : {}", ganache.endpoint());
    
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "wallet first address: {}",
        first_address.encode_hex::<String>()
    );

    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
    
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("wallet first balance: {}", first_balance);

    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!(
        "Balance for address {}: {}",
        other_address_hex, other_balance
    );

    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);

    let receipt = provider 
         .send_transaction(tx,None)
         .await?
         .long_msg("Pending transfer")
         .confirmation(1)
         .await?
         .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("cannot get black number ")?
    );
    println!(
        "Balance of {} {}",
        other_address_hex,
        provider.get.balance(other_address, None).await?
    );
    Ok(())     
}