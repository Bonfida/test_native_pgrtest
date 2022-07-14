### Example test case of program test native code execution

Then running `cargo test` on this project will give you an example test case of account deserialization error in native mode.

With program-test from the solana version of:

- ORIGINAL origin/master branch of https://github.com/solana-labs/solana.git:
  It should execute successfully.

- FORK bonfida_fork/fix_program_test of https://github.com/Bonfida/solana:
  First clone the fork and indicate its location the Cargo.toml patch section of this projects toml and run `cargo update`. (Uncomment the patch section)
  Running `cargo test` with this fork should give undefined behavior, being successfull, returning `invalid memory reference` or `NotEnoughAccountKeys`.
  The error seems to come from the account deserialization in program-test, it will only give such a behaviour with this exact number of accounts passed to the CPI.
