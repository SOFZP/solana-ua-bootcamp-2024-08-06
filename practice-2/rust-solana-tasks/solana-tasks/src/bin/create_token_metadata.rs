use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use spl_token::solana_program::system_instruction;
use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::pda::find_metadata_account;
use mpl_token_metadata::state::Creator;
use std::str::FromStr;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key).expect("Invalid secret key format");
    let payer = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    let mint_pubkey = Pubkey::from_str("2w9q9N4KeqzGoTJ8wSuNFCcs2Wd7ztTVqbV5diAzg6XA").expect("Invalid mint pubkey");
    let (metadata_pubkey, _) = find_metadata_account(&mint_pubkey); // Візьмемо лише Pubkey, ігноруючи інший елемент

    let token_metadata_instruction = create_metadata_accounts_v3(
        mpl_token_metadata::id(),
        metadata_pubkey,
        mint_pubkey,
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        "VIK test3 token".to_string(),          // Назва токена
        "VIK-T3".to_string(),              // Символ токена
        "https://rose-rotten-condor-732.mypinata.cloud/ipfs/QmU1w6kvoex7LjTUk4A3TZ5FTdcVqEMdE9gkR9VHp4WXEw".to_string(), // URI до метаданих
        Some(vec![Creator {
            address: payer.pubkey(),
            verified: true,
            share: 100,
        }]),
        0,
        true,
        false,
        None,
        None,
        None,
    );

    let mut transaction = Transaction::new_with_payer(
        &[token_metadata_instruction],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
    transaction.sign(&[&payer], recent_blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Metadata created! Transaction Signature: {:?}", signature),
        Err(err) => eprintln!("Failed to send and confirm transaction: {:?}", err),
    }
}
