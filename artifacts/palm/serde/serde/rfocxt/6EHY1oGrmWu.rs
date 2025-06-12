use crate::lib::*;
use crate::de::value::{BorrowedBytesDeserializer, BytesDeserializer};
use crate::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error, IntoDeserializer,
    VariantAccess, Visitor,
};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::de::{MapAccess, Unexpected};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::content::{
    Content, ContentDeserializer, ContentRefDeserializer, EnumDeserializer,
    InternallyTaggedUnitVisitor, TagContentOtherField, TagContentOtherFieldVisitor,
    TagOrContentField, TagOrContentFieldVisitor, TaggedContentVisitor,
    UntaggedUnitVisitor,
};
pub use crate::seed::InPlaceSeed;
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! forward_to_deserialize_other {
    ($($func:ident ($($arg:ty),*))*) => {
        $(fn $func < V > (self, $(_ : $arg,)* _visitor : V) -> Result < V::Value,
        Self::Error > where V : Visitor <'de >, { Self::deserialize_other() })*
    };
}
#[derive(Debug, Clone)]
pub enum Content<'de> {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),
    None,
    Some(Box<Content<'de>>),
    Unit,
    Newtype(Box<Content<'de>>),
    Seq(Vec<Content<'de>>),
    Map(Vec<(Content<'de>, Content<'de>)>),
}
impl<'de> Content<'de> {
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Content::Str(x) => Some(x),
            Content::String(ref x) => Some(x),
            Content::Bytes(x) => str::from_utf8(x).ok(),
            Content::ByteBuf(ref x) => str::from_utf8(x).ok(),
            _ => None,
        }
    }
    #[cold]
    fn unexpected(&self) -> Unexpected {}
}
#[cfg(any(feature = "std", feature = "alloc"))]
fn flat_map_take_entry<'de>(
    entry: &mut Option<(Content<'de>, Content<'de>)>,
    recognized: &[&str],
) -> Option<(Content<'de>, Content<'de>)> {
    let is_recognized = match entry {
        None => false,
        Some((k, _v)) => k.as_str().map_or(false, |name| recognized.contains(&name)),
    };
    if is_recognized { entry.take() } else { None }
}
