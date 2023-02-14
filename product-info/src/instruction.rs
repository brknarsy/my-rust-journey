use solana_program::{program_error::ProgramError};
use borsh::BorshDeserialize;
pub enum ProductInstruction {
    AddProductData {
        name: String,
        price: u64,
        quantity: u64,
    },
    UpdateProductData {
        name: String,
        price: u64,
        quantity: u64,
    },
    DeleteProductData {
        name: String,
    },
}

#[derive(BorshDeserialize)]
struct ProductPayload {
    name: String,
    price: u64,
    quantity: u64,
}

impl ProductInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = ProductPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddProductData {
                name: payload.name,
                price: payload.price,
                quantity: payload.quantity,
            },
            1 => Self::UpdateProductData {
                name: payload.name,
                price: payload.price,
                quantity: payload.quantity,
            },
            2 => Self::DeleteProductData {
                name: payload.name,
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}