use pliron::{
    context::Context,
    dialect::{Dialect, DialectName},
};

pub mod attributes;
pub mod from_clif_ir;
pub mod op_interfaces;
pub mod ops;

pub fn register(ctx: &mut Context) {
    let dialect = Dialect::new(DialectName::new("clif"));
    dialect.register(ctx);
    ops::register(ctx);
}
