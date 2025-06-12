use crate::lib::*;
use crate::actually_private;
use crate::de::value::{MapDeserializer, SeqDeserializer};
use crate::de::{
    self, size_hint, Deserialize, DeserializeSeed, Deserializer, EnumAccess, Expected,
    IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor,
};
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
fn visit_content_seq_ref<'a, 'de, V, E>(
    content: &'a [Content<'de>],
    visitor: V,
) -> Result<V::Value, E>
where
    V: Visitor<'de>,
    E: de::Error,
{
    let mut seq_visitor = SeqDeserializer::new(content.iter());
    let value = tri!(visitor.visit_seq(& mut seq_visitor));
    tri!(seq_visitor.end());
    Ok(value)
}
