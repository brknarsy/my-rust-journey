use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg};

pub mod instruction;
use instruction::{MovieInstruction};

entrypoint!(process_instruction);

pub fn process_instruction (
    _program_id: Pubkey,
    _accounts: &[AccountInfo],
    _isntruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World!");
    Ok(())
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Adding Movie Review...")
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    Ok(())
}

pub fn process_instruction(
    program_idÇ: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)?;

    match instruction {
        MovieInstruction::AddMovieReview { title, rating, description } => {
            add_movie_review(program_id, accounts, title, rating, description)
        }
    }
}
