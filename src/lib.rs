#![crate_type = "rlib"]
#![deny(warnings)]
#![deny(bad_style)]

extern crate value_trait;
extern crate simd_json;

pub use object_builder::ObjectBuilder;
pub use array_builder::ArrayBuilder;
pub use serializer::{Serializer, ObjectSerializer, ObjectScopeSerializer};
pub use array_serializer::ArraySerializer;

pub mod array_builder;
pub mod object_builder;
pub mod serializer;
pub mod array_serializer;

pub fn array<F>(builder: F) -> ArrayBuilder where F: FnOnce(&mut ArrayBuilder) {
    ArrayBuilder::build(builder)
}

pub fn object<F>(builder: F) -> ObjectBuilder where F: FnOnce(&mut ObjectBuilder) {
    ObjectBuilder::build(builder)
}
