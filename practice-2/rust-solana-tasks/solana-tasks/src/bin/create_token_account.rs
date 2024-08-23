use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::{
    create_associated_token_account,
};
use std::env;
use std::str::FromStr;

fn main() {
    // Отримання секретного ключа з .env файлу
    dotenv::dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key).expect("Invalid secret key format");
    let payer = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    // Встановлення RPC з'єднання
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Адреса токенового мінту
    let mint_pubkey = Pubkey::from_str("2w9q9N4KeqzGoTJ8wSuNFCcs2Wd7ztTVqbV5diAzg6XA").expect("Invalid mint pubkey");

    // Адреса власника токенового акаунта
    let owner_pubkey = payer.pubkey();

    // Створення транзакції для створення акаунта
    let transaction = Transaction::new_signed_with_payer(
        &[create_associated_token_account(
            &payer.pubkey(),
            &owner_pubkey,
            &mint_pubkey,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        client.get_latest_blockhash().unwrap(),
    );

    // Надсилання транзакції
    match client.send_and_confirm_transaction_with_spinner_and_config(
    &transaction,
    CommitmentConfig::confirmed(), // Додайте цей аргумент
    RpcSendTransactionConfig {
        skip_preflight: true,
        ..RpcSendTransactionConfig::default()
    },
) {
    Ok(signature) => println!("Token Account Creation Tx: {:?}", signature),
    Err(err) => eprintln!("Failed to send and confirm transaction: {:?}", err),
};
}
