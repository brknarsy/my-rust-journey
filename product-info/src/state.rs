use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProductAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub price: u64,
    pub quantity: u64,
}