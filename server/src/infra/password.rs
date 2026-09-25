use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};

#[cfg(test)]
use mockall::automock;
use thiserror::Error;

use crate::constants::{
    PASSWORD_MEMORY_COST_BYTES, PASSWORD_PARALLELISM, PASSWORD_TIME_COST_PASSES,
};

#[derive(Debug, Clone, Error)]
pub enum PasswordServiceError {
    #[error("error hasing password: {0}")]
    HashError(String),

    #[error("error parsing hash: {0}")]
    ParseError(String),
}

#[cfg_attr(test, automock)]
pub trait PasswordService {
    fn hash_password(&self, password: &str) -> Result<String, PasswordServiceError>;
    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, PasswordServiceError>;
}

#[derive(Clone)]
pub struct Argon2Service;

impl PasswordService for Argon2Service {
    fn hash_password(&self, password: &str) -> Result<String, PasswordServiceError> {
        let params = Params::new(
            PASSWORD_MEMORY_COST_BYTES,
            PASSWORD_TIME_COST_PASSES,
            PASSWORD_PARALLELISM,
            None,
        )
        .unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        argon2
            .hash_password(password.as_bytes())
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
