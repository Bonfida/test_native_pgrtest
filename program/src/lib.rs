use bonfida_utils::declare_id_with_central_state;

#[doc(hidden)]
pub mod entrypoint;
#[doc(hidden)]
pub mod error;
/// Program instructions and their CPI-compatible bindings
pub mod instruction;

#[doc(hidden)]
pub(crate) mod processor;

#[allow(missing_docs)]
pub mod cpi;

declare_id_with_central_state!("GgFVMhNxYwJ5FNU6b7Vpzta3envkFh5p8EBQTycsSWKm");
