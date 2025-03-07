// Import required types from the Pliron framework:
// - `Context`: The IR context in which dialects and ops are registered.
// - `Dialect` and `DialectName`: Used to define and name new dialects.
use pliron::{
    context::Context,
    dialect::{Dialect, DialectName},
};

// Declare sub-modules for this dialect:
// - `attributes`: Contains attribute definitions for the dialect.
// - `from_clif_ir`: Provides functionality to convert from CLIF IR to Pliron IR.
// - `op_interfaces`: Defines interfaces for operations (ops) in this dialect.
// - `ops`: Implements the actual operations.
pub mod attributes;
pub mod from_clif_ir;
pub mod op_interfaces;
pub mod ops;

/// Registers the CLIF dialect and its associated operations into the provided context.
///
/// # Arguments
///
/// * `ctx` - A mutable reference to the current IR context.
///
/// # Process
///
/// 1. Creates a new dialect named "clif" using the `DialectName::new` constructor.
/// 2. Registers the newly created dialect with the context so that it can be recognized.
/// 3. Calls the registration function in the `ops` module to register all CLIF operations.
///
/// This setup is essential for enabling the use and parsing of CLIF-based operations in the IR.
pub fn register(ctx: &mut Context) {
    // Create a new dialect with the name "clif"
    let dialect = Dialect::new(DialectName::new("clif"));
    // Register the dialect into the given context
    dialect.register(ctx);
    // Register all operations defined in the `ops` module into the context
    ops::register(ctx);
}
