use std::fmt;

use argon2::{
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::constants::{
    PASSWORD_MEMORY_COST_BYTES, PASSWORD_PARALLELISM, PASSWORD_TIME_COST_PASSES,
};

pub(crate) enum PasswordServiceError {
    HashError(String),
    ParseError(String),
}

impl fmt::Display for PasswordServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordServiceError::HashError(e) => write!(f, "error hasing password: {}", e),
            PasswordServiceError::ParseError(e) => write!(f, "error parsing hash: {}", e),
        }
    }
}

pub(crate) trait PasswordService {
    fn hash_password(&self, password: &str) -> Result<String, PasswordServiceError>;
    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, PasswordServiceError>;
}

#[derive(Clone)]
pub(crate) struct Argon2Service;

impl PasswordService for Argon2Service {
    fn hash_password(&self, password: &str) -> Result<String, PasswordServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(
            PASSWORD_MEMORY_COST_BYTES,
            PASSWORD_TIME_COST_PASSES,
            PASSWORD_PARALLELISM,
            None,
        )
        .unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| PasswordServiceError::HashError(e.to_string()))
    }

    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, PasswordServiceError> {
        let params = Params::new(
            PASSWORD_MEMORY_COST_BYTES,
            PASSWORD_TIME_COST_PASSES,
            PASSWORD_PARALLELISM,
            None,
        )
        .unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        let password_hash = PasswordHash::new(password_hash)
            .map_err(|e| PasswordServiceError::ParseError(e.to_string()))?;
        Ok(argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok())
    }
}
