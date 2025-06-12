use crate::lib::*;
use crate::actually_private;
use crate::de::value::{MapDeserializer, SeqDeserializer};
use crate::de::{
    self, size_hint, Deserialize, DeserializeSeed, Deserializer, EnumAccess, Expected,
    IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor,
};
pub trait Visitor<'de>: Sized {
    type Value;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Bool(v), &self))
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Signed(v), &self))
    }
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut buf = [0u8; 58];
        let mut writer = crate::format::Buf::new(&mut buf);
        fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as i128", v))
            .unwrap();
        Err(Error::invalid_type(Unexpected::Other(writer.as_str()), &self))
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Unsigned(v), &self))
    }
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut buf = [0u8; 57];
        let mut writer = crate::format::Buf::new(&mut buf);
        fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as u128", v))
            .unwrap();
        Err(Error::invalid_type(Unexpected::Other(writer.as_str()), &self))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_f64(v as f64)
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Float(v), &self))
    }
    #[inline]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v.encode_utf8(&mut [0u8; 4]))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Str(v), &self))
    }
    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v)
    }
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Bytes(v), &self))
    }
    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_bytes(v)
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_bytes(&v)
    }
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Option, &self))
    }
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(Unexpected::Option, &self))
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Unit, &self))
    }
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(Unexpected::NewtypeStruct, &self))
    }
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let _ = seq;
        Err(Error::invalid_type(Unexpected::Seq, &self))
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let _ = map;
        Err(Error::invalid_type(Unexpected::Map, &self))
    }
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let _ = data;
        Err(Error::invalid_type(Unexpected::Enum, &self))
    }
    fn __private_visit_untagged_option<D>(self, _: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Err(())
    }
}
pub trait DeserializeSeed<'de>: Sized {
    type Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}
struct TagOrContentVisitor<'de> {
    name: &'static str,
    value: PhantomData<TagOrContent<'de>>,
}
struct ContentVisitor<'de> {
    value: PhantomData<Content<'de>>,
}
pub enum TagOrContent<'de> {
    Tag,
    Content(Content<'de>),
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
impl<'de> Visitor<'de> for TagOrContentVisitor<'de> {
    type Value = TagOrContent<'de>;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {}
    fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_bool(value).map(TagOrContent::Content)
    }
    fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_i8(value).map(TagOrContent::Content)
    }
    fn visit_i16<F>(self, value: i16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_i16(value).map(TagOrContent::Content)
    }
    fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_i32(value).map(TagOrContent::Content)
    }
    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_i64(value).map(TagOrContent::Content)
    }
    fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_u8(value).map(TagOrContent::Content)
    }
    fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_u16(value).map(TagOrContent::Content)
    }
    fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_u32(value).map(TagOrContent::Content)
    }
    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_u64(value).map(TagOrContent::Content)
    }
    fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_f32(value).map(TagOrContent::Content)
    }
    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_f64(value).map(TagOrContent::Content)
    }
    fn visit_char<F>(self, value: char) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_char(value).map(TagOrContent::Content)
    }
    fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if value == self.name {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new().visit_str(value).map(TagOrContent::Content)
        }
    }
    fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if value == self.name {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new().visit_borrowed_str(value).map(TagOrContent::Content)
        }
    }
    fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if value == self.name {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new().visit_string(value).map(TagOrContent::Content)
        }
    }
    fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if value == self.name.as_bytes() {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new().visit_bytes(value).map(TagOrContent::Content)
        }
    }
    fn visit_borrowed_bytes<F>(self, value: &'de [u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if value == self.name.as_bytes() {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new().visit_borrowed_bytes(value).map(TagOrContent::Content)
        }
    }
    fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        if value == self.name.as_bytes() {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new().visit_byte_buf(value).map(TagOrContent::Content)
        }
    }
    fn visit_unit<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_unit().map(TagOrContent::Content)
    }
    fn visit_none<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        ContentVisitor::new().visit_none().map(TagOrContent::Content)
    }
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        ContentVisitor::new().visit_some(deserializer).map(TagOrContent::Content)
    }
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        ContentVisitor::new()
            .visit_newtype_struct(deserializer)
            .map(TagOrContent::Content)
    }
    fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        ContentVisitor::new().visit_seq(visitor).map(TagOrContent::Content)
    }
    fn visit_map<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        ContentVisitor::new().visit_map(visitor).map(TagOrContent::Content)
    }
    fn visit_enum<V>(self, visitor: V) -> Result<Self::Value, V::Error>
    where
        V: EnumAccess<'de>,
    {
        ContentVisitor::new().visit_enum(visitor).map(TagOrContent::Content)
    }
}
impl<'de> ContentVisitor<'de> {
    fn new() -> Self {
        ContentVisitor {
            value: PhantomData,
        }
    }
}
