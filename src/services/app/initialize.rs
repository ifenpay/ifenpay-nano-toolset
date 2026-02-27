use std::fs;
use std::path::Path;
use dotenvy::dotenv;
use rand::{rng};
use rand::distr::{Distribution, Uniform};

use crate::handlers::wallet::create_wallet_api;
use crate::services::wallet::wallet::encrypt_wallet_data;


pub async fn initialize_rust_app() {
	let env_file = Path::new(".env");
	let env_example = Path::new(".env-example");
	if !env_file.exists() {
		println!("⚠️  No .env file found - creating from .env-example");
		if env_example.exists() {
			fs::copy(env_example, env_file).expect("Failed to copy .env-example to .env");
            let charset: Vec<char> =
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+=-"
                    .chars()
                    .collect();

            let mut rng = rng();
            let dist = Uniform::new(0, charset.len()).unwrap();

            let random_password = format!(
                "AUTOGEN_{}",
                (0..40)
                    .map(|_| charset[dist.sample(&mut rng)])
                    .collect::<String>()
            );
			let content = fs::read_to_string(env_file).unwrap_or_default();
			let quoted_password = format!("'{}'", random_password);
			let replaced = content.replace("ChangeMe123", &quoted_password);
			fs::write(env_file, replaced).expect("Failed to write .env file");
			println!("✅ .env created with auto-generated AI_WALLET_PASSWORD!");
			println!("🔐 Password: {}", random_password);
			println!("⚠️  IMPORTANT: Password saved in .env file");
		} else {
			println!("❌ .env-example not found, cannot create .env");
		}

	} 
	dotenv().ok();


	let wallets_dir = Path::new("data");
	fs::create_dir_all(wallets_dir).expect("Failed to create wallets directory");

	let wallet_file = wallets_dir.join("ai.bin");
	if !wallet_file.exists() {
		let wallet_data = create_wallet_api().await.expect("Failed to create wallet via API").0.data.expect("Failed to parse wallet data");
		let wallet_password = std::env::var("AI_WALLET_PASSWORD").expect("AI_WALLET_PASSWORD not set in .env");
		let encrypted_wallet = encrypt_wallet_data(&wallet_data, &wallet_password).expect("Failed to encrypt wallet data");

		fs::write(&wallet_file, &encrypted_wallet).expect("Failed to create data/ai.bin");
		println!("✅ Created data/ai.bin");
	}
}