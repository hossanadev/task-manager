use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use tracing::error;

pub fn hash_password(plain_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    match argon.hash_password(plain_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            error!("Failed to hash password");
            panic!("Failed to hash password");
        }
    }
}