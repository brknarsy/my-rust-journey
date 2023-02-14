use solana_program::{
    msg,
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Sealed},
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use borsh::BorshSerialize;
use crate::error::ReviewError;
use crate::instruction::ProductInstruction;
use crate::state::ProductAccountState;
use std::convert::TryInto;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProductInstruction::unpack(instruction_data)?;
    match instruction {
        ProductInstruction::AddProductData {
            name,
            price,
            quantity,
        } => add_product_data(program_id, accounts, name, price, quantity),
        ProductInstruction::UpdateProductData {
            name,
            price,
            quantity,
        } => update_product_data(program_id, accounts, name, price, quantity),
        ProductInstruction::DeleteProductData { name } => {
            delete_product_data(program_id, accounts, name)
        }
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

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature)
    }
    
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), name.as_bytes().as_ref()],
        program_id,
    );

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument)
    }
    
    let account_len: usize = 1000;
    let total_len: usize = 1 + (4 + name.len()) + 64 + 64;
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into())
    }
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
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            name.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    let mut product_data =
        try_from_slice_unchecked::<ProductAccountState>(&pda_account.data.borrow()).unwrap();
    product_data.name = name;
    product_data.price = price;
    product_data.quantity = quantity;
    product_data.is_initialized = true;

    product_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn update_product_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    price: u64,
    quantity: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner)
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature)
    }

    let mut product_data =
        try_from_slice_unchecked::<ProductAccountState>(&pda_account.data.borrow()).unwrap();

    if !product_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }
    
    let (pda, _bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), product_data.name.as_bytes().as_ref(),], program_id);

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into())
    }

    let total_len: usize = 1 + 64 + 64 + (4 + product_data.name.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into())
    }
    
    product_data.price = price;
    product_data.quantity = quantity;

    product_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn delete_product_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    Ok(())
}
