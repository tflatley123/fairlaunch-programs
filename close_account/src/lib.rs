extern crate solana_program;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn closewallet(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let closing = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = 0usize;
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    let diff = closing.lamports() - lamports_required;

    // Send the rent back to the payer
    **closing.lamports.borrow_mut() -= diff;
    **payer.lamports.borrow_mut() += diff;

    // Realloc the account to zero
    closing.realloc(account_span, true)?;

    // Assign the account to the System Program
    closing.assign(system_program.key);

    Ok(())
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    closewallet(accounts)
}

solana_program::entrypoint!(process_instruction);
