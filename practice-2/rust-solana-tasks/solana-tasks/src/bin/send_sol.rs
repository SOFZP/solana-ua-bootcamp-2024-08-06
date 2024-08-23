use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
};
use std::env;

fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let secret_key = env::var("SECRET_KEY").expect("Missing SECRET_KEY in .env");
    let sender = Keypair::from_base58_string(&secret_key);

    let recipient_pubkey = Pubkey::from_str("6FVoYDbUXD1EaHcJfCBrdA6A9ch8JYzz4BJb7EzcFssf").unwrap();

    let lamports = 0.0108 * solana_sdk::native_token::LAMPORTS_PER_SOL;

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &sender.pubkey(),
            &recipient_pubkey,
            lamports as u64,
        )],
        Some(&sender.pubkey()),
        &[&sender],
        client.get_recent_blockhash().unwrap().0,
    );

    match client.send_and_confirm_transaction(&tx) {
        Ok(signature) => println!("Transaction confirmed, signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    };
}
