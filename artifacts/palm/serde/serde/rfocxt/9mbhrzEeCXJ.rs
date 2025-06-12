use crate::lib::*;
use crate::actually_private;
use crate::de::value::{MapDeserializer, SeqDeserializer};
use crate::de::{
    self, size_hint, Deserialize, DeserializeSeed, Deserializer, EnumAccess, Expected,
    IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor,
};
pub trait VariantAccess<'de>: Sized {
    type Error: Error;
    fn unit_variant(self) -> Result<(), Self::Error>;
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>;
    #[inline]
    fn newtype_variant<T>(self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.newtype_variant_seed(PhantomData)
    }
    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}
struct VariantRefDeserializer<'a, 'de: 'a, E>
where
    E: de::Error,
{
    value: Option<&'a Content<'de>>,
    err: PhantomData<E>,
}
pub struct ContentRefDeserializer<'a, 'de: 'a, E> {
    content: &'a Content<'de>,
    err: PhantomData<E>,
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
impl<'de, 'a, E> de::VariantAccess<'de> for VariantRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;
    fn unit_variant(self) -> Result<(), E> {
        match self.value {
            Some(value) => {
                de::Deserialize::deserialize(ContentRefDeserializer::new(value))
            }
            None => Ok(()),
        }
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, E>
    where
        T: de::DeserializeSeed<'de>,
    {}
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
            Some(other) => {
                Err(de::Error::invalid_type(other.unexpected(), &"tuple variant"))
            }
            None => {
                Err(
                    de::Error::invalid_type(
                        de::Unexpected::UnitVariant,
                        &"tuple variant",
                    ),
                )
            }
        }
    }
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Some(Content::Map(v)) => visit_content_map_ref(v, visitor),
            Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
            Some(other) => {
                Err(de::Error::invalid_type(other.unexpected(), &"struct variant"))
            }
            None => {
                Err(
                    de::Error::invalid_type(
                        de::Unexpected::UnitVariant,
                        &"struct variant",
                    ),
                )
            }
        }
    }
}
impl<'a, 'de, E> ContentRefDeserializer<'a, 'de, E> {
    pub fn new(content: &'a Content<'de>) -> Self {
        ContentRefDeserializer {
            content,
            err: PhantomData,
        }
    }
}
