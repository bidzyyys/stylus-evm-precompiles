#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::prelude::{entrypoint, external, sol_storage};

pub mod evm;

use evm::ECDSA;

sol_storage! {
    #[entrypoint]
    struct Playground {
        #[borrow]
        ECDSA ecdsa;
    }
}

#[external]
#[inherit(ECDSA)]
impl Playground {
    fn test(&self) -> u64 {
        0
    }
}
