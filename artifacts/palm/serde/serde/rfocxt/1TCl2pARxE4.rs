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
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Unexpected<'a> {
    /// The input contained a boolean value that was not expected.
    Bool(bool),
    /// The input contained an unsigned integer `u8`, `u16`, `u32` or `u64` that
    /// was not expected.
    Unsigned(u64),
    /// The input contained a signed integer `i8`, `i16`, `i32` or `i64` that
    /// was not expected.
    Signed(i64),
    /// The input contained a floating point `f32` or `f64` that was not
    /// expected.
    Float(f64),
    /// The input contained a `char` that was not expected.
    Char(char),
    /// The input contained a `&str` or `String` that was not expected.
    Str(&'a str),
    /// The input contained a `&[u8]` or `Vec<u8>` that was not expected.
    Bytes(&'a [u8]),
    /// The input contained a unit `()` that was not expected.
    Unit,
    /// The input contained an `Option<T>` that was not expected.
    Option,
    /// The input contained a newtype struct that was not expected.
    NewtypeStruct,
    /// The input contained a sequence that was not expected.
    Seq,
    /// The input contained a map that was not expected.
    Map,
    /// The input contained an enum that was not expected.
    Enum,
    /// The input contained a unit variant that was not expected.
    UnitVariant,
    /// The input contained a newtype variant that was not expected.
    NewtypeVariant,
    /// The input contained a tuple variant that was not expected.
    TupleVariant,
    /// The input contained a struct variant that was not expected.
    StructVariant,
    /// A message stating what uncategorized thing the input contained that was
    /// not expected.
    ///
    /// The message should be a noun or noun phrase, not capitalized and without
    /// a period. An example message is "unoriginal superhero".
    Other(&'a str),
}
impl<'de, 'a, E> de::VariantAccess<'de> for VariantRefDeserializer<'a, 'de, E>
where
    E: de::Error,
{
    type Error = E;
    fn unit_variant(self) -> Result<(), E> {}
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
impl<'de> Content<'de> {
    pub fn as_str(&self) -> Option<&str> {}
    #[cold]
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::Bool(b) => Unexpected::Bool(b),
            Content::U8(n) => Unexpected::Unsigned(n as u64),
            Content::U16(n) => Unexpected::Unsigned(n as u64),
            Content::U32(n) => Unexpected::Unsigned(n as u64),
            Content::U64(n) => Unexpected::Unsigned(n),
            Content::I8(n) => Unexpected::Signed(n as i64),
            Content::I16(n) => Unexpected::Signed(n as i64),
            Content::I32(n) => Unexpected::Signed(n as i64),
            Content::I64(n) => Unexpected::Signed(n),
            Content::F32(f) => Unexpected::Float(f as f64),
            Content::F64(f) => Unexpected::Float(f),
            Content::Char(c) => Unexpected::Char(c),
            Content::String(ref s) => Unexpected::Str(s),
            Content::Str(s) => Unexpected::Str(s),
            Content::ByteBuf(ref b) => Unexpected::Bytes(b),
            Content::Bytes(b) => Unexpected::Bytes(b),
            Content::None | Content::Some(_) => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            Content::Newtype(_) => Unexpected::NewtypeStruct,
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
        }
    }
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
fn visit_content_map_ref<'a, 'de, V, E>(
    content: &'a [(Content<'de>, Content<'de>)],
    visitor: V,
) -> Result<V::Value, E>
where
    V: Visitor<'de>,
    E: de::Error,
{
    fn content_ref_deserializer_pair<'a, 'de>(
        (k, v): &'a (Content<'de>, Content<'de>),
    ) -> (&'a Content<'de>, &'a Content<'de>) {
        (k, v)
    }
    let map = content.iter().map(content_ref_deserializer_pair);
    let mut map_visitor = MapDeserializer::new(map);
    let value = tri!(visitor.visit_map(& mut map_visitor));
    tri!(map_visitor.end());
    Ok(value)
}
