use crate::common::utils::sign_send_instructions;
use solana_program::system_program;
use test_native_pgrtest::{
    central_state,
    entrypoint::process_instruction,
    instruction::{
        example,
        example_instr::{Accounts, Params},
    },
};

use {
    solana_program_test::{processor, ProgramTest},
    solana_sdk::{
        account::Account,
        signer::{keypair::Keypair, Signer},
    },
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

    // program_test.add_program("example_dependency", example_dependency::ID, None);

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
        Accounts {
            system_program: &system_program::ID,
            fee_payer: &alice.pubkey(),
            example_state: &central_state::KEY,
            // example_state2: &central_state::KEY,
            // example_state3: &central_state::KEY,
            // example_state4: &central_state::KEY,
            // example_state5: &central_state::KEY,
        },
        Params {},
    );

    sign_send_instructions(&mut prg_test_ctx, vec![instruction], vec![&alice])
        .await
        .unwrap();
}
