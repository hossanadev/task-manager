use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

const SECRET: &[u8] = b"SUPER_SECURE_SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: String) -> String {
    let expiration = Utc::now() + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
        .unwrap()
}

pub fn validate_token(token: &str) -> Option<String> {
    let validation = Validation::default();

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &validation,
    );

    decoded.ok().map(|data| data.claims.sub)
}