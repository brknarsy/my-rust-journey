use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};

pub mod instruction;
use instruction::{StudentInstruction};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StudentInstruction::unpack(instruction_data)?;
    match instruction {
        StudentInstruction::CreateStudent {name, message} => {
            create_student(program_id, account_info, name, message);
        }
    }
}

pub fn create_student (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    msg!("Creating Student");

    msg!("Student Name: {}", name);
    msg!("Student Message: {}", message);
    Ok(())
}