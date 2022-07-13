use {
    num_derive::FromPrimitive,
    solana_program::{decode_error::DecodeError, program_error::ProgramError},
    thiserror::Error,
};

#[derive(Clone, Debug, Error, FromPrimitive)]
pub enum TestNativePgrtestInnerError {
    #[error("This account is already initialized")]
    AlreadyInitialized,
    #[error("Data type mismatch")]
    DataTypeMismatch,
    #[error("Wrong account owner")]
    WrongOwner,
    #[error("Account is uninitialized")]
    Uninitialized,
}

impl From<TestNativePgrtestInnerError> for ProgramError {
    fn from(e: TestNativePgrtestInnerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TestNativePgrtestInnerError {
    fn type_of() -> &'static str {
        "TestNativePgrtestInnerError"
    }
}
