use std::fmt::Debug;

use newtype_derive::Newtype;

#[derive(Clone, Debug, PartialEq, Newtype)]
pub struct Username(String);
