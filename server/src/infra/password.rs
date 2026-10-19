use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};

#[cfg(test)]
use mockall::automock;
use secrecy::ExposeSecret;
use thiserror::Error;

use crate::{
    constants::{PASSWORD_MEMORY_COST_BYTES, PASSWORD_PARALLELISM, PASSWORD_TIME_COST_PASSES},
    domain::secret_types::Password,
};

#[derive(Debug, Clone, Error)]
pub enum PasswordServiceError {
    #[error("error hashing password: {0}")]
    Hashing(String),

    #[error("error parsing hash: {0}")]
    Parsing(String),
}

#[cfg_attr(test, automock)]
pub trait PasswordService {
    fn hash_password(
        &self,
        password: &Password,
    ) -> Result<crate::domain::secret_types::PasswordHash, PasswordServiceError>;
    fn verify_password(
        &self,
        password: &Password,
        password_hash: &crate::domain::secret_types::PasswordHash,
    ) -> Result<bool, PasswordServiceError>;
}

#[derive(Clone)]
pub struct Argon2Service;

impl PasswordService for Argon2Service {
    fn hash_password(
        &self,
        password: &Password,
    ) -> Result<crate::domain::secret_types::PasswordHash, PasswordServiceError> {
        let params = Params::new(
            PASSWORD_MEMORY_COST_BYTES,
            PASSWORD_TIME_COST_PASSES,
            PASSWORD_PARALLELISM,
            None,
        )
        .unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        argon2
            .hash_password(password.as_ref().expose_secret().as_bytes())
            .map(|h| crate::domain::secret_types::PasswordHash::new(h.to_string()))
            .map_err(|e| PasswordServiceError::Hashing(e.to_string()))
    }

    fn verify_password(
        &self,
        password: &Password,
        password_hash: &crate::domain::secret_types::PasswordHash,
    ) -> Result<bool, PasswordServiceError> {
        let params = Params::new(
            PASSWORD_MEMORY_COST_BYTES,
            PASSWORD_TIME_COST_PASSES,
            PASSWORD_PARALLELISM,
            None,
        )
        .unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        let password_hash = PasswordHash::new(password_hash.as_ref().expose_secret())
            .map_err(|e| PasswordServiceError::Parsing(e.to_string()))?;
        Ok(argon2
            .verify_password(password.as_ref().expose_secret().as_bytes(), &password_hash)
            .is_ok())
    }
}
