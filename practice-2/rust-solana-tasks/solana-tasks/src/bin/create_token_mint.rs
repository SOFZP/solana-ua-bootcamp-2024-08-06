use dotenv::dotenv;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use std::env;
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::system_instruction;
use serde_json;
use solana_sdk::program_pack::Pack;

fn main() {
    dotenv().ok();

    // Завантаження секретного ключа з .env файлу
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");

    // Декодування приватного ключа з секретного ключа
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key).expect("Invalid secret key format");
    let payer = Keypair::from_bytes(&secret_key_bytes).expect("Failed to create keypair from secret key");

    // Підключення до Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Генерація нової пари ключів для мінта токенів
    let mint = Keypair::new();

    // Отримання балансу для оренди
    let rent_exemption_balance = client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .expect("Failed to get minimum balance for rent exemption");

    // Отримання blockhash
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Створення транзакції для мінта токенів
    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                rent_exemption_balance,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                &payer.pubkey(),
                None,
                2,
            )
            .expect("Failed to create initialize_mint instruction"),
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );

    // Відправка та підтвердження транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Token Mint Pubkey: {:?}", mint.pubkey()),
        Err(err) => eprintln!("Failed to send and confirm transaction: {:?}", err),
    }
}

