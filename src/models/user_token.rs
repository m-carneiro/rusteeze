use std::env;
use chrono::Utc;
use log::debug;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Serialize, Deserialize};
use crate::models::user::LoginInfoDTO;

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub issued_at: i64,
    pub expiration: i64,
    pub user: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        dotenv::dotenv().expect("Failed to read .env file");
        let max_age: i64 = match env::var("MAX_AGE") {
            Ok(val) => val.parse::<i64>().unwrap_or(ONE_WEEK),
            Err(_) => ONE_WEEK,
        };

        debug!("Max age: {}", max_age);

        let now = Utc::now().timestamp_nanos_opt() / 1_000_000_000;
        let payload = UserToken {
            issued_at: now,
            expiration: now + max_age,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .unwrap()
    }
}