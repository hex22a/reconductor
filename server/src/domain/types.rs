use std::fmt::Debug;

use newtype_derive::Newtype;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
pub struct Hostname(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
pub struct OsMatch(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
pub struct OsAccuracy(i32);

#[derive(Clone, Debug, PartialEq, Newtype, sqlx::Type)]
#[sqlx(transparent)]
pub struct Username(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Vendor(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ProjectId(Uuid);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PortId(Uuid);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct HostId(Uuid);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PortNumber(i32);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ScanRunId(Uuid);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PortProtocol(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct State(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Service(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Product(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Version(String);

#[derive(Clone, Debug, PartialEq, Newtype, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct CpeString(String);
