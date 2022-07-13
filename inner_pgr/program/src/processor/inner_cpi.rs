//! Example instruction

use crate::central_state;
use solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
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
        };

        Ok(accounts)
    }
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _params: Params) -> ProgramResult {
    let accounts = Accounts::parse(accounts, program_id)?;

    invoke(
        &system_instruction::transfer(
            accounts.fee_payer.key,
            accounts.example_state.key,
            1_000_000_000,
        ),
        &[
            accounts.fee_payer.clone(),
            accounts.example_state.clone(),
            accounts.system_program.clone(),
        ],
    )?;

    invoke_signed(
        &system_instruction::allocate(accounts.example_state.key, 100),
        &[
            accounts.example_state.clone(),
            accounts.system_program.clone(),
        ],
        &[&[&program_id.to_bytes(), &[central_state::NONCE]]],
    )?;

    invoke_signed(
        &system_instruction::assign(accounts.example_state.key, program_id),
        &[
            accounts.example_state.clone(),
            accounts.system_program.clone(),
        ],
        &[&[program_id.to_bytes().as_slice(), &[central_state::NONCE]]],
    )?;

    //Edit a bit of data
    let data_len = accounts.example_state.data_len();
    let mut account_data = accounts.example_state.data.borrow_mut();
    account_data.copy_from_slice(vec![1; data_len].as_slice());

    Ok(())
}
