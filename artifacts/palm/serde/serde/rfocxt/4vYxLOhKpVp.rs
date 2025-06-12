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
pub trait MapAccess<'de> {
    type Error: Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>;
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>;
    #[inline]
    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        match tri!(self.next_key_seed(kseed)) {
            Some(key) => {
                let value = tri!(self.next_value_seed(vseed));
                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }
    #[inline]
    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.next_key_seed(PhantomData)
    }
    #[inline]
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(PhantomData)
    }
    #[inline]
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        self.next_entry_seed(PhantomData, PhantomData)
    }
    #[inline]
    fn size_hint(&self) -> Option<usize> {
        None
    }
}
#[cfg(any(feature = "std", feature = "alloc"))]
struct FlatStructAccess<'a, 'de: 'a, E> {
    iter: slice::IterMut<'a, Option<(Content<'de>, Content<'de>)>>,
    pending_content: Option<Content<'de>>,
    fields: &'static [&'static str],
    _marker: PhantomData<E>,
}
pub struct ContentDeserializer<'de, E> {
    content: Content<'de>,
    err: PhantomData<E>,
}
pub struct T;
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
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, 'de, E> MapAccess<'de> for FlatStructAccess<'a, 'de, E>
where
    E: Error,
{
    type Error = E;
    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        for entry in self.iter.by_ref() {
            if let Some((key, content)) = flat_map_take_entry(entry, self.fields) {
                self.pending_content = Some(content);
                return seed.deserialize(ContentDeserializer::new(key)).map(Some);
            }
        }
        Ok(None)
    }
    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.pending_content.take() {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err(Error::custom("value is missing")),
        }
    }
}
impl<'de, E> ContentDeserializer<'de, E> {
    pub fn new(content: Content<'de>) -> Self {
        ContentDeserializer {
            content,
            err: PhantomData,
        }
    }
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
