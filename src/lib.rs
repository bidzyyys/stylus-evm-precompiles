#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::prelude::{entrypoint, external, sol_storage};

pub mod evm;

sol_storage! {
    #[entrypoint]
    struct Playground {}
}

#[external]
impl Playground {
    pub fn test(&self) -> u64 {
        0
    }
}
