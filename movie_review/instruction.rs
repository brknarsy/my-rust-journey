use solana_program::{program_error::ProgramError};
use borsh::{BorshDeserialize};

pub enum StudentInstruction {
    CreateStudent {
        name: String,
        message: String
    }
}

#[derive(BorshDeserialize)]
struct StudentInstructionPayload {
    name: String,
    message: String
}

impl StudentInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentInstructionPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::CreateStudent {
                name: payload.name,
                message: payload.message
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}