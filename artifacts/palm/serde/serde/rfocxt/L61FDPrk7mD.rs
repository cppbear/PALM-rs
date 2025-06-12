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
pub struct TagOrContentFieldVisitor {
    /// Name of the tag field of the adjacently tagged enum
    pub tag: &'static str,
    /// Name of the content field of the adjacently tagged enum
    pub content: &'static str,
}
pub enum TagOrContentField {
    Tag,
    Content,
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
impl<'de> Visitor<'de> for TagOrContentFieldVisitor {
    type Value = TagOrContentField;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {}
    fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match field_index {
            0 => Ok(TagOrContentField::Tag),
            1 => Ok(TagOrContentField::Content),
            _ => Err(de::Error::invalid_value(Unexpected::Unsigned(field_index), &self)),
        }
    }
    fn visit_str<E>(self, field: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if field == self.tag {
            Ok(TagOrContentField::Tag)
        } else if field == self.content {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(field), &self))
        }
    }
    fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if field == self.tag.as_bytes() {
            Ok(TagOrContentField::Tag)
        } else if field == self.content.as_bytes() {
            Ok(TagOrContentField::Content)
        } else {
            Err(de::Error::invalid_value(Unexpected::Bytes(field), &self))
        }
    }
}
