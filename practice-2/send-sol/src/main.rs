use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
};
use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use serde_json;

fn main() {
    dotenv().ok(); // Завантажити .env файл

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let secret_key = env::var("SECRET_KEY").expect("Missing SECRET_KEY in .env");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key).expect("Invalid secret key format");
    let sender = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair");

    let recipient_pubkey = Pubkey::from_str("6FVoYDbUXD1EaHcJfCBrdA6A9ch8JYzz4BJb7EzcFssf").unwrap();

    let lamports = (0.01_f64 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64;

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &sender.pubkey(),
            &recipient_pubkey,
            lamports,
        )],
        Some(&sender.pubkey()),
        &[&sender],
        client.get_latest_blockhash().unwrap(),
    );

    match client.send_and_confirm_transaction(&tx) {
        Ok(signature) => println!("Transaction confirmed, signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    };
}

