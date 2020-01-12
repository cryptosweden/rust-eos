//! TODO docs
use crate::{SerializeData, NumBytes, Read, Write};
use serde::{Deserialize, Serialize};

/// TODO docs
// TODO Read, Write, NumBytes needs a custom implementation based on fixed_bytes
#[derive(
    Read,
    Write,
    NumBytes,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[eosio_core_root_path = "crate"]
pub struct Checksum256([u8; 32]);
impl SerializeData for Checksum256 {}

impl Checksum256 {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl From<[u8; 32]> for Checksum256 {
    #[inline]
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<Checksum256> for [u8; 32] {
    #[inline]
    fn from(value: Checksum256) -> Self {
        value.0
    }
}
