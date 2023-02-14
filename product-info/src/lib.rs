use solana_program::{
    msg,
    entrypoint,
    entrypoint::ProgramResult,
    system_instruction,
    borsh::{try_from_slice_unchecked},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    account_info::{next_account_info, AccountInfo}, program::invoke_signed,
};

use std::convert::TryInto;

pub mod instruction;
pub mod state;
use instruction::ProductInstruction;
use state::{ProductAccountState};
use borsh::BorshSerialize;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProductInstruction::unpack(instruction_data)?;
    match instruction {
        ProductInstruction::AddProductData { name, price, quantity } => {
            add_product_data(program_id, accounts, name, price, quantity)
        },
        ProductInstruction::UpdateProductData { name, price, quantity } => {
            update_product_data(program_id, accounts, name, price, quantity)
        },
        ProductInstruction::DeleteProductData { name } => {
            delete_product_data(program_id, accounts, name)
        },
    }
}

pub fn add_product_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    price: u64,
    quantity: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), name.as_bytes().as_ref()], program_id);
    
    let account_len: usize = 1 + 8 + 8 + name.len();
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ), 
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        &[&[initializer.key.as_ref(), name.as_bytes().as_ref(), &[bump_seed]]])?;
    
    let mut product_data = try_from_slice_unchecked::<ProductAccountState>(&pda_account.data.borrow()).unwrap();
    product_data.name = name;
    product_data.price = price;
    product_data.quantity = quantity;
    product_data.is_initialized = true;

    product_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn update_product_data (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    price: u64,
    quantity: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let mut product_data = try_from_slice_unchecked::<ProductAccountState>(&pda_account.data.borrow()).unwrap();
    product_data.name = name;
    product_data.price = price;
    product_data.quantity = quantity;

    product_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn delete_product_data (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String
) -> ProgramResult {
    Ok(())
}