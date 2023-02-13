use borsh::{BoschDeserialize};
use solana_program::{program_error::ProgramError};

pub enum NoteInstruction {
    CreateNote {
        id: u64,
        title: String,
        body: String,
    },
    UpdateNote {
        id: u64,
        title: String,
        body: String,
    },
    DeleteNote [
        id: u64,
    ],
}

#[derive(BoschDeserialize)]
struct NoteInstructionPayload {
    id: u64,
    title: String,
    body: String,
}

impl NoteInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        let payload = NoteInstructionPayload::try_from_slice(rest).unwrap();
    }

    Ok(match variant {
        0 => Self::CreateNote {
            id: payload.id,
            title: payload.title,
            body: payload.body,
        },
        1 => Self::UpdateNote {
            id: payload.id,
            title: payload.title,
            body: payload.body,
        },
        2 => Self::DeleteNote { 
            id: payload.id 
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    })
}