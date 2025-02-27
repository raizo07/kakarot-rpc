use crate::providers::eth_provider::error::EthereumDataFormatError;
use reth_primitives::{Address, B256, U256, U64};
use starknet::core::types::{EthAddress, Felt};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Felt252Wrapper(Felt);

impl From<Felt> for Felt252Wrapper {
    fn from(felt: Felt) -> Self {
        Self(felt)
    }
}

impl From<Felt252Wrapper> for Felt {
    fn from(felt: Felt252Wrapper) -> Self {
        felt.0
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<Address> for Felt252Wrapper {
    fn from(address: Address) -> Self {
        // safe unwrap since H160 is 20 bytes
        Self(Felt::from_bytes_be_slice(address.as_slice()))
    }
}

impl From<U64> for Felt252Wrapper {
    fn from(value: U64) -> Self {
        value.to::<u64>().into()
    }
}

impl From<u64> for Felt252Wrapper {
    fn from(value: u64) -> Self {
        Self(value.into())
    }
}

impl From<u128> for Felt252Wrapper {
    fn from(value: u128) -> Self {
        Self(Felt::from(value))
    }
}

impl TryFrom<Felt252Wrapper> for Address {
    type Error = EthereumDataFormatError;

    fn try_from(felt: Felt252Wrapper) -> Result<Self, Self::Error> {
        EthAddress::from_felt(&felt)
            .map(|eth_address| Self::from_slice(eth_address.as_bytes()))
            .map_err(|_| EthereumDataFormatError::Primitive)
    }
}

impl From<B256> for Felt252Wrapper {
    fn from(value: B256) -> Self {
        Self(Felt::from_bytes_be(value.as_ref()))
    }
}

impl From<U256> for Felt252Wrapper {
    fn from(u256: U256) -> Self {
        Self(Felt::from_bytes_be(&u256.to_be_bytes()))
    }
}

impl From<Felt252Wrapper> for U256 {
    fn from(felt: Felt252Wrapper) -> Self {
        Self::from_be_bytes(felt.to_bytes_be())
    }
}

impl Deref for Felt252Wrapper {
    type Target = Felt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Felt252Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This macro provides a convenient way to convert a value from a source
/// type $val that implements Into<Felt252Wrapper> into a target type that
/// implements From<Felt252Wrapper>.
#[macro_export]
macro_rules! into_via_wrapper {
    ($val: expr) => {{
        let intermediate: $crate::models::felt::Felt252Wrapper = $val.into();
        intermediate.into()
    }};
}

/// This macro provides a convenient way to convert a value from a source
/// type $val that implements `TryInto`<Felt252Wrapper> into a target type that
/// implements From<Felt252Wrapper>.
#[macro_export]
macro_rules! into_via_try_wrapper {
    ($val: expr) => {{
        let intermediate: Result<_, $crate::providers::eth_provider::error::EthereumDataFormatError> =
            TryInto::<$crate::models::felt::Felt252Wrapper>::try_into($val)
                .map_err(|_| $crate::providers::eth_provider::error::EthereumDataFormatError::Primitive)
                .map(Into::into);
        intermediate
    }};
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use hex::FromHex;

    use super::*;

    // 2**160 - 1
    const MAX_ADDRESS: &str = "ffffffffffffffffffffffffffffffffffffffff";
    // 2**160
    const OVERFLOW_ADDRESS: &str = "010000000000000000000000000000000000000000";

    // 2**251 + 17 * 2**192 + 1
    const OVERFLOW_FELT: &str = "0800000000000011000000000000000000000000000000000000000000000001";

    #[test]
    fn test_address_try_from_felt_should_pass() {
        // Given
        let address: Felt252Wrapper = Felt::from_hex(MAX_ADDRESS).unwrap().into();

        // When
        let address = Address::try_from(address).unwrap();

        // Then
        let expected_address = <[u8; 20]>::from_hex(MAX_ADDRESS).unwrap();
        assert_eq!(expected_address, address.0);
    }

    #[test]
    #[should_panic(expected = "Primitive")]
    fn test_address_try_from_felt_should_fail() {
        // Given
        let address: Felt252Wrapper = Felt::from_hex(OVERFLOW_ADDRESS).unwrap().into();

        // When
        Address::try_from(address).unwrap();
    }

    #[test]
    fn test_felt_try_from_b256_should_pass() {
        // Given
        let hash = B256::from_slice(&Felt::MAX.to_bytes_be());

        // When
        let hash = Felt252Wrapper::from(hash);

        // Then
        let expected_hash = Felt::MAX;
        assert_eq!(expected_hash, hash.0);
    }

    #[test]
    fn test_felt_try_from_b256_should_fail() {
        // Given
        let hash = B256::from_str(OVERFLOW_FELT).unwrap();

        // When
        assert_eq!(Felt252Wrapper::from(hash).0, Felt::ZERO,);
    }

    #[test]
    fn test_felt_try_from_u256_should_pass() {
        // Given
        let hash = U256::try_from_be_slice(&Felt::MAX.to_bytes_be()).unwrap();

        // When
        let hash = Felt252Wrapper::from(hash);

        // Then
        let expected_hash = Felt::MAX;
        assert_eq!(expected_hash, hash.0);
    }

    #[test]
    fn test_felt_try_from_u256_should_fail() {
        // Given
        let hash = U256::from_str_radix(OVERFLOW_FELT, 16).unwrap();

        // When
        assert_eq!(Felt252Wrapper::from(hash).0, Felt::ZERO,);
    }
}
