use std::env;

use once_cell::sync::Lazy;

pub static IFENPAY_API_URL: Lazy<String> = Lazy::new(|| {env::var("IFENPAY_API_URL").expect("IFENPAY_API_URL must be set")});