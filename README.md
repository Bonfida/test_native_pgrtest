### Example test case of program test native code execution

Then running `cargo test` on this project will give you an example test case for the account resizing via CPI in native mode.

With program-test from the solana version of:

- origin/master branch of https://github.com/solana-labs/solana.git:
  It will fail with a timeout followed by: `thread 'solana-bank-forks-client' panicked at 'Account data resizing not supported yet: 0 -> 10. Consider making this test conditional on `#[cfg(feature = "test-bpf")]`', ~/.cargo/registry/src/github.com-1ecc6299db9ec823/solana-program-test-1.10.12/src/lib.rs:335:13`

- bonfida_fork/fix_program_test of https://github.com/Bonfida/solana:
  First clone the fork and indicate its location the Cargo.toml patch section of this project.
  Running this partially fixed version will work perfectly on this repo, yet this is not the intent of this program.
