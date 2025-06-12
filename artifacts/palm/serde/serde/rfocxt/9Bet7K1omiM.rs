use crate::lib::*;
use crate::actually_private;
use crate::de::value::{MapDeserializer, SeqDeserializer};
use crate::de::{
    self, size_hint, Deserialize, DeserializeSeed, Deserializer, EnumAccess, Expected,
    IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor,
};
pub trait EnumAccess<'de>: Sized {
    type Error: Error;
    type Variant: VariantAccess<'de, Error = Self::Error>;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>;
    #[inline]
    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.variant_seed(PhantomData)
    }
}
pub struct EnumDeserializer<'de, E>
where
    E: de::Error,
{
    variant: Content<'de>,
    value: Option<Content<'de>>,
    err: PhantomData<E>,
}
pub enum Content {
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
    Bytes(Vec<u8>),
    None,
    Some(Box<Content>),
    Unit,
    UnitStruct(&'static str),
    UnitVariant(&'static str, u32, &'static str),
    NewtypeStruct(&'static str, Box<Content>),
    NewtypeVariant(&'static str, u32, &'static str, Box<Content>),
    Seq(Vec<Content>),
    Tuple(Vec<Content>),
    TupleStruct(&'static str, Vec<Content>),
    TupleVariant(&'static str, u32, &'static str, Vec<Content>),
    Map(Vec<(Content, Content)>),
    Struct(&'static str, Vec<(&'static str, Content)>),
    StructVariant(&'static str, u32, &'static str, Vec<(&'static str, Content)>),
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
impl<'de, E> EnumDeserializer<'de, E>
where
    E: de::Error,
{
    pub fn new(
        variant: Content<'de>,
        value: Option<Content<'de>>,
    ) -> EnumDeserializer<'de, E> {
        EnumDeserializer {
            variant,
            value,
            err: PhantomData,
        }
    }
}
