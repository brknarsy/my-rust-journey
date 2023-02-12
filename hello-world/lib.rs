use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg};

entrypoint!(process_instruction);

pub fn process_instruction (
    _program_id: Pubkey,
    _accounts: &[AccountInfo],
    _isntruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World!");
    Ok(())
}