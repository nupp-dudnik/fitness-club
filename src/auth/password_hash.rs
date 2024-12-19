use anyhow::Result;
use argon2::{
  password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};

pub fn hash_password(password: &str) -> Result<String> {
  let salt = SaltString::generate(&mut OsRng);

  let argon2 = Argon2::default();

  let password_hash = argon2
    .hash_password(password.as_bytes(), &salt)
    .map_err(|e| anyhow::anyhow!("Hashing failed: {e}"))?
    .to_string();

  Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<()> {
  let parsed_hash = PasswordHash::new(password_hash)
    .map_err(|e| anyhow::anyhow!("Failed parsing the hash: {e}"))?;

  Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash)
    .map_err(|e| anyhow::anyhow!("Failed password verification: {e}"))
}
