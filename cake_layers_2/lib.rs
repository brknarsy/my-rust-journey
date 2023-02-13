use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    msg
};

pub mod instruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    let instruction = NoteInstruction::unpack(instruction_data)?;

    match instruction {
        NoteInstruction::Createnote {id, title, body} => {

        },
        NoteInstruction::UpdateNote (id, title, body) => {

        },
        NoteInstruction::DeleteNote (id) => {

        }
    }

    Ok(())
}
