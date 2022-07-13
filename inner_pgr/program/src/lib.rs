use bonfida_utils::declare_id_with_central_state;

#[doc(hidden)]
pub mod entrypoint;
#[doc(hidden)]
pub mod error;
/// Program instructions and their CPI-compatible bindings
pub mod instruction;

#[doc(hidden)]
pub mod processor;

declare_id_with_central_state!("B3dXF6BNwFSpfASDw4JkDiXRXR3bTit4tg2LooJ948Hq");
