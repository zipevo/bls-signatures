use crate::{
    bip32_keys::{BIP32ExtendedPublicKey, BIP32_EXTENDED_PUBLIC_KEY_SIZE},
    BlsError,
};

impl BIP32ExtendedPublicKey {
    pub fn from_bytes_legacy(bytes: &[u8]) -> Result<Self, BlsError> {
        Self::from_bytes_with_legacy_flag(bytes, true)
    }

    pub fn public_child_legacy(&self, index: u32) -> Self {
        self.public_child_with_legacy_flag(index, true)
    }

    pub fn serialize_legacy(&self) -> Box<[u8; BIP32_EXTENDED_PUBLIC_KEY_SIZE]> {
        self.serialize_with_legacy_flag(true)
    }
}
