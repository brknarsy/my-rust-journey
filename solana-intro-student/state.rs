use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct StudentAccountState {
    pub is_initialized: bool,
    pub name: String,
    pub message: String
}