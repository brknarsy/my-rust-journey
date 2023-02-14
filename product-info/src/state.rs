use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::{IsInitialized, Sealed};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProductAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub price: u64,
    pub quantity: u64,
}

impl Sealed for ProductAccountState {}

impl IsInitialized for ProductAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
