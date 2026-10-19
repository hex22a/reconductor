use newtype_derive::Newtype;
use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};

use crate::infra::persistence::kv::StorageId;

#[derive(Clone, Debug, Newtype)]
pub struct CsrfCookie(SecretString);

#[derive(Clone, Debug, Newtype)]
pub struct CsrfToken(SecretString);

impl StorageId for CsrfToken {
    fn storage_id(&self) -> String {
        let digest = Sha256::digest(self.as_ref().expose_secret().as_bytes());
        hex::encode(digest)
    }
}

#[derive(Clone, Debug, Newtype)]
pub struct Password(SecretString);

#[derive(Clone, Debug, Newtype)]
pub struct PasswordHash(SecretString);

impl<T: sqlx::Database> sqlx::Type<T> for PasswordHash
where
    str: sqlx::Type<T>,
{
    fn type_info() -> T::TypeInfo {
        <str as sqlx::Type<T>>::type_info()
    }
    fn compatible(ty: &T::TypeInfo) -> bool {
        <str as sqlx::Type<T>>::compatible(ty)
    }
}

impl<'r, T: sqlx::Database> sqlx::Decode<'r, T> for PasswordHash
where
    &'r str: sqlx::Decode<'r, T>,
{
    fn decode(value: T::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<T>>::decode(value)?;
        Ok(Self(SecretString::from(s.to_owned())))
    }
}

impl<'q, T: sqlx::Database> sqlx::Encode<'_, T> for PasswordHash
where
    for<'a> &'a str: sqlx::Encode<'q, T>,
{
    fn encode_by_ref(
        &self,
        buf: &mut T::ArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <&str as sqlx::Encode<T>>::encode_by_ref(&self.0.expose_secret(), buf)
    }
}
#[derive(Clone, Debug, Newtype)]
pub struct SessionToken(SecretString);

impl StorageId for SessionToken {
    fn storage_id(&self) -> String {
        let digest = Sha256::digest(self.as_ref().expose_secret().as_bytes());
        hex::encode(digest)
    }
}
