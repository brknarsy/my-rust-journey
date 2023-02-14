use solana_program::{
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    entrypoint,
    entrypoint::ProgramResult,
    program::{invoke_signed},
    borsh::{try_from_slice_unchecked},  
    pubkey::Pubkey,
    msg
};

use std::convert::TryInto;

pub mod state;
pub mod instruction;

use instruction::{StudentInstruction};
use state::{StudentAccountState};
use borsh::BorshSerialize;

entrypoint!(process_instruction);


pub fn create_student (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), name.as_bytes().as_ref()], program_id);

    let account_len: usize = 1 + (4 + name.len()) + (4 + message.len());
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        //instruction
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        //accounts
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        //seeds
        &[&[initializer.key.as_ref(), name.as_bytes().as_ref(), &[bump_seed]]],
    )?;

    msg!("PDA account created: {}", pda);

    msg!("Unpacking state account");
    let mut student_data = try_from_slice_unchecked::<StudentAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Unpacked state account");

    student_data.name = name;
    student_data.message = message;
    student_data.is_initialized = true;

    msg!("Serializing account");
    student_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Serialized account");
    Ok(())
}


pub fn process_instruction(
    program_id: &Pubkey,
    account_info: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StudentInstruction::unpack(instruction_data)?;
    match instruction {
        StudentInstruction::CreateStudent {name, message} => {
            create_student(program_id, account_info, name, message)
        }
    }
}