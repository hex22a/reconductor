use ipnetwork::IpNetwork;
use newtype_derive::Newtype;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ScanId(Uuid);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ScanTarget(IpNetwork);
