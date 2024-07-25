use alloy_primitives::{address, Address, FixedBytes};
use alloy_sol_types::sol;
use stylus_proc::{sol_interface, sol_storage, SolidityError};
use stylus_sdk::{call::Call, prelude::external, storage::TopLevelStorage};

const ECRECOVER_ADDR: Address =
    address!("0000000000000000000000000000000000000001");

sol! {
    /// The signature derives the `Address::ZERO`.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ECDSAInvalidSignature();
}

/// An error that occurred in the implementation of an [`ECDSA`] library.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// The signature derives the `Address::ZERO`.
    InvalidSignature(ECDSAInvalidSignature),
}

sol_interface! {
    /// EVM Precompiles interface.
    ///
    /// Interface for any contract that wants to call `ecrecover` precompile .
    interface EVMPrecompile {
        #[allow(missing_docs)]
        function ecrecover(
            bytes32 hash,
            uint8 v,
            bytes32 r,
            bytes32 s
        ) returns (address);
    }
}

sol_storage! {
    /// ECDSA contract.
    pub struct ECDSA {}
}

/// NOTE: Implementation of [`TopLevelStorage`] to be able use `&mut self` when
/// calling other contracts and not `&mut (impl TopLevelStorage +
/// BorrowMut<Self>)`. Should be fixed in the future by the Stylus team.
unsafe impl TopLevelStorage for ECDSA {}

#[external]
impl ECDSA {
    /// Calls `ecrecover` EVM precompile.
    /// The `ecrecover` EVM precompile allows for malleable (non-unique)
    /// signatures: this function rejects them by requiring the `s` value to
    /// be in the lower half order, and the `v` value to be either 27 or 28.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `hash` - Hash of the message.
    /// * `v` - `v` value from the signature.
    /// * `r` - `r` value from the signature.
    /// * `s` - `s` value from the signature.
    ///
    /// # Errors
    ///
    /// * If the `s` value is grater than `EIP2_VALUE`, then the error
    /// [`Error::ECDSAInvalidSignatureS`] is returned.
    /// * If the recovered address is `Address::ZERO`, then the error
    /// [`Error::InvalidSignature`] is returned.
    ///
    /// # Panics
    ///
    /// * If `ecrecover` precompile fail to execute.
    fn recover(
        &mut self,
        hash: FixedBytes<32>,
        v: u8,
        r: FixedBytes<32>,
        s: FixedBytes<32>,
    ) -> Result<Address, Error> {
        let call = Call::new_in(self);
        let recovered = EVMPrecompile::new(ECRECOVER_ADDR)
            .ecrecover(call, hash, v, r, s)
            .expect("should call `ecrecover` precompile");

        // if recovered.is_zero() {
        // return Err(ECDSAInvalidSignature {}.into());
        // }
        Ok(recovered)
    }
}
