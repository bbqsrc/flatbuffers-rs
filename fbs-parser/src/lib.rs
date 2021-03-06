mod lexer;

#[doc(inline)]
pub use lexer::{
    ident, parse_path, Attribute, Enum, EnumValue, Error, Field, FileExtension, FileIdentifier,
    Ident, Include, Literal, Metadata, Namespace, Primitive, RootType, Schema, SchemaType, Struct,
    Table, Type, Union, UnionValue, Scalar, SingleValue,
};
