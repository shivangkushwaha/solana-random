use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::clock::Clock,
    sysvar::Sysvar,
};

// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RandomNumber {
    pub number: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Random Number Generator Program Entry Point");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to store the random number
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the current slot (block height)
    let clock = Clock::get()?;
    let slot = clock.slot;

    // Create a pseudo-random number using the slot and some custom data
    let seed = u64::from_le_bytes(instruction_data.try_into().unwrap_or_default());
    let random_number = (slot ^ seed) % 100; // Generate a number between 0 and 99

    msg!("Generated random number: {}", random_number);

    // Store the random number in the account data
    let mut rng_account = RandomNumber::try_from_slice(&account.data.borrow())?;
    rng_account.number = random_number as u32;
    rng_account.serialize(&mut *account.data.borrow_mut())?;

    Ok(())
}
