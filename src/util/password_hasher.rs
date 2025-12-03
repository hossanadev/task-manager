use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

const HASH_PASSWORD_ERROR_MESSAGE: &str = "Failed to hash password";

pub fn hash_password(plain: &str) -> Result<String, &'static str> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    argon
        .hash_password(plain.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| HASH_PASSWORD_ERROR_MESSAGE)
}