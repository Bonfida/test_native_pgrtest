use crate::common::utils::sign_send_instructions;
use solana_program::system_program;
use solana_sdk::account::Account;
use test_native_pgrtest::{
    entrypoint::process_instruction,
    instruction::example,
    processor::example_instr::{self},
};

use {
    solana_program_test::{processor, ProgramTest},
    solana_sdk::signer::{keypair::Keypair, Signer},
};

pub mod common;

#[tokio::test]
async fn test() {
    // Create program and test environment
    let alice = Keypair::new();

    let mut program_test = ProgramTest::new(
        "test_native_pgrtest",
        test_native_pgrtest::ID,
        processor!(process_instruction),
    );

    program_test.add_program(
        "dd",
        test_native_pgrtest_inner::ID,
        processor!(test_native_pgrtest_inner::entrypoint::process_instruction),
    );

    program_test.add_account(
        alice.pubkey(),
        Account {
            lamports: 100_000_000_000,
            ..Account::default()
        },
    );

    ////
    // Create test context
    ////
    let mut prg_test_ctx = program_test.start_with_context().await;

    let instruction = example(
        example_instr::Accounts {
            inner_program: &test_native_pgrtest_inner::ID,
            system_program: &system_program::ID,
            fee_payer: &alice.pubkey(),
            example_state: &test_native_pgrtest_inner::central_state::KEY,
        },
        example_instr::Params {},
    );

    sign_send_instructions(&mut prg_test_ctx, vec![instruction], vec![&alice])
        .await
        .unwrap();
}
