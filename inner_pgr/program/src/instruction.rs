use bonfida_utils::InstructionsAccount;

use crate::processor::inner_cpi;
use {
    borsh::{BorshDeserialize, BorshSerialize},
    num_derive::FromPrimitive,
    solana_program::{instruction::Instruction, pubkey::Pubkey},
};
#[allow(missing_docs)]
#[derive(BorshDeserialize, BorshSerialize, FromPrimitive)]
pub enum ProgramInstruction {
    /// The inner nested instr
    ///
    /// | Index | Writable | Signer | Description                   |
    /// | --------------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The system program account    |
    /// | 1     | ❌        | ❌      | The SPL token program account |
    /// | 2     | ✅        | ✅      | Fee payer account             |
    InnerCpi,
}

#[allow(missing_docs)]
pub fn inner(accounts: inner_cpi::Accounts<Pubkey>, params: inner_cpi::Params) -> Instruction {
    accounts.get_instruction(crate::ID, ProgramInstruction::InnerCpi as u8, params)
}
