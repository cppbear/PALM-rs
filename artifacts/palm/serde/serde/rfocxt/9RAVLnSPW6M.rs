use crate::lib::*;
use crate::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
pub const FIELDS: &[&str] = &["start"];
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
pub struct RangeFromVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
impl<'de, Idx> Visitor<'de> for RangeFromVisitor<Idx>
where
    Idx: Deserialize<'de>,
{
    type Value = Idx;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {}
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let start: Idx = match tri!(seq.next_element()) {
            Some(value) => value,
            None => {
                return Err(Error::invalid_length(0, &self));
            }
        };
        Ok(start)
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut start: Option<Idx> = None;
        while let Some(key) = tri!(map.next_key()) {
            match key {
                Field::Start => {
                    if start.is_some() {
                        return Err(<A::Error as Error>::duplicate_field("start"));
                    }
                    start = Some(tri!(map.next_value()));
                }
            }
        }
        let start = match start {
            Some(start) => start,
            None => return Err(<A::Error as Error>::missing_field("start")),
        };
        Ok(start)
    }
}
