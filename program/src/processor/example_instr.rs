//! Example instruction

use crate::central_state;
use solana_program::{program::invoke_signed, system_program};
use test_native_pgrtest_inner::{instruction::inner, processor::inner_cpi};

use {
    bonfida_utils::{BorshSize, InstructionsAccount},
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    #[cons(writable, signer)]
    /// The fee payer account
    pub fee_payer: &'a T,

    #[cons(writable)]
    /// The example state account
    pub example_state: &'a T,

    /// The inner program to be called
    pub inner_program: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        accounts: &'a [AccountInfo<'b>],
        _program_id: &Pubkey,
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Accounts {
            system_program: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
            example_state: next_account_info(accounts_iter)?,
            inner_program: next_account_info(accounts_iter)?,
        };

        Ok(accounts)
    }
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _params: Params) -> ProgramResult {
    let accounts = Accounts::parse(accounts, program_id)?;

    let inner_instr = inner(
        inner_cpi::Accounts {
            system_program: &system_program::ID,
            fee_payer: accounts.fee_payer.key,
            example_state: accounts.example_state.key,
            example_state2: accounts.example_state.key,
        },
        inner_cpi::Params {},
    );
    invoke_signed(
        &inner_instr,
        &[
            accounts.inner_program.clone(),
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.example_state.clone(),
            accounts.example_state.clone(),
        ],
        &[&[program_id.to_bytes().as_slice(), &[central_state::NONCE]]],
    )
    .unwrap();

    Ok(())
}
