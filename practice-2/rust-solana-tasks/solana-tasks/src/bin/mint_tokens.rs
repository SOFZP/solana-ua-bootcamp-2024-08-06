use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::mint_to;
use std::str::FromStr;
use std::env;

fn main() {
    // Завантажуємо приватний ключ з .env
    dotenv::dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");

    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key).expect("Invalid secret key format");
    let payer = Keypair::from_bytes(&secret_key_bytes).expect("Invalid keypair");

    // RPC URL Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), solana_sdk::commitment_config::CommitmentConfig::confirmed());

    // Адреса токенового мінта та акаунта, в який будемо мінтити токени
    let mint_pubkey = Pubkey::from_str("2w9q9N4KeqzGoTJ8wSuNFCcs2Wd7ztTVqbV5diAzg6XA").expect("Invalid mint pubkey");
    let token_account_pubkey = Pubkey::from_str("3jJEWt5bPkEsi7db3Z7inzEwZ2db6LRQbvhDBrqHSfNN").expect("Invalid token account pubkey");

    // Створюємо інструкцію для мінтингу
    let mint_to_ix = mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &token_account_pubkey,
        &payer.pubkey(),
        &[],
        990, // Скільки токенів мінтити (у мінт-деномінації)
    ).expect("Failed to create mint_to instruction");

    // Створюємо транзакцію
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Відправляємо транзакцію
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Minted tokens! Transaction Signature: {:?}", signature),
        Err(err) => eprintln!("Failed to mint tokens: {:?}", err),
    }
}
