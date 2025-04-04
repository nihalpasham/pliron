use pliron::{
    attribute::Attribute,
    context::Context,
    derive::{def_attribute, format_attribute},
    impl_verify_succ,
    parsable::Parsable,
};

/// Flag attributes for memory operations like load/store.
///
/// In addition, flags also determine the endianness of the memory access.  By default,
/// any memory access uses the native endianness determined by the target ISA.  
#[def_attribute("clif.memflags")]
#[format_attribute]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MemFlagsAttr {
    Aligned,      // * 0 - aligned flag
    ReadOnly,     // * 1 - readonly flag
    LittleEndian, // * 2 - little endian flag
    BigEndian,    // * 3 - big endian flag
    Checked,      // * 4 - checked flag
    AliasRegion,  // * 5/6 - alias region
    TrapCode,     // * 7/8/9/10/11/12/13/14 - trap code
    Unallocated,  // * 15 - unallocated
}

impl_verify_succ!(MemFlagsAttr);

/// A 32-bit signed immediate offset as an attribute.
///
/// This is used to encode an immediate offset for load/store instructions. All supported Cranelift ISAs have
/// a maximum load/store offset that fits in an `i32`.
#[def_attribute("clif.offset32")]
#[format_attribute("`<` $0 `>`")]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Offset32Attr(pub i32);
impl_verify_succ!(Offset32Attr);

pub fn register(ctx: &mut Context) {
    MemFlagsAttr::register_attr_in_dialect(ctx, MemFlagsAttr::parser_fn);
    Offset32Attr::register_attr_in_dialect(ctx, Offset32Attr::parser_fn);
}
