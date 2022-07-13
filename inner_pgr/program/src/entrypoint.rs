use crate::{error::TestNativePgrtestInnerError, processor::Processor};

use {
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, decode_error::DecodeError, entrypoint::ProgramResult, msg,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// The entrypoint to the program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TestNativePgrtestInnerError>();
        return Err(error);
    }
    Ok(())
}

impl PrintProgramError for TestNativePgrtestInnerError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            TestNativePgrtestInnerError::AlreadyInitialized => {
                msg!("Error: This account is already initialized")
            }
            TestNativePgrtestInnerError::DataTypeMismatch => msg!("Error: Data type mismatch"),
            TestNativePgrtestInnerError::WrongOwner => msg!("Error: Wrong account owner"),
            TestNativePgrtestInnerError::Uninitialized => msg!("Error: Account is uninitialized"),
        }
    }
}
