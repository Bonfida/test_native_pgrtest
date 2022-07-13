use {
    borsh::BorshDeserialize,
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use solana_program::pubkey;

use crate::instruction::ProgramInstruction;

pub mod example_instr;

pub const INNER_PGR_ID: Pubkey = pubkey!("B3dXF6BNwFSpfASDw4JkDiXRXR3bTit4tg2LooJ948Hq");

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = FromPrimitive::from_u8(instruction_data[0])
            .ok_or(ProgramError::InvalidInstructionData)?;
        let instruction_data = &instruction_data[1..];
        msg!("Instruction unpacked");

        match instruction {
            ProgramInstruction::ExampleInstr => {
                msg!("Instruction: Example Instruction");
                let params = example_instr::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                example_instr::process(program_id, accounts, params)?;
            }
        }

        Ok(())
    }
}
