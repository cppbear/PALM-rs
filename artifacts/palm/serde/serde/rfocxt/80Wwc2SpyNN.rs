use crate::lib::*;
use crate::ser::{Error, Impossible, Serialize, Serializer};
macro_rules! fmt_primitives {
    ($($f:ident : $t:ty,)*) => {
        $(fn $f (self, v : $t) -> fmt::Result { Display::fmt(& v, self) })*
    };
}
pub trait Serializer: Sized {
    type Ok;
    type Error: Error;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTuple: SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleStruct: SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleVariant: SerializeTupleVariant<
            Ok = Self::Ok,
            Error = Self::Error,
        >;
    type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStructVariant: SerializeStructVariant<
            Ok = Self::Ok,
            Error = Self::Error,
        >;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("i128 is not supported"))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("u128 is not supported"))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let mut iter = iter.into_iter();
        let mut serializer = tri!(self.serialize_seq(iterator_len_hint(& iter)));
        tri!(iter.try_for_each(| item | serializer.serialize_element(& item)));
        serializer.end()
    }
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut iter = iter.into_iter();
        let mut serializer = tri!(self.serialize_map(iterator_len_hint(& iter)));
        tri!(
            iter.try_for_each(| (key, value) | serializer.serialize_entry(& key, &
            value))
        );
        serializer.end()
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.serialize_str(&value.to_string())
    }
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display;
    #[inline]
    fn is_human_readable(&self) -> bool {
        true
    }
}
pub trait Pair {
    type First;
    type Second;
    fn split(self) -> (Self::First, Self::Second);
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
    fn size_hint(&self) -> Option<usize>;
}
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Deserialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>;
}
pub trait IdentifierDeserializer<'de, E: Error> {
    type Deserializer: Deserializer<'de, Error = E>;
    fn from(self) -> Self::Deserializer;
}
pub trait Expected {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
}
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        note = "for local types consider adding `#[derive(serde::Serialize)]` to your `{Self}` type",
        note = "for types from other crates check whether the crate offers a `serde` feature flag",
    )
)]
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
pub trait DeserializeSeed<'de>: Sized {
    type Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}
pub trait SeqAccess<'de> {
    type Error: Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>;
    #[inline]
    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.next_element_seed(PhantomData)
    }
    #[inline]
    fn size_hint(&self) -> Option<usize>;
}
pub trait IntoDeserializer<'de, E: Error = value::Error> {
    type Deserializer: Deserializer<'de, Error = E>;
    fn into_deserializer(self) -> Self::Deserializer;
}
pub struct Impossible<Ok, Error> {
    void: Void,
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}
#[derive(Debug)]
pub struct Error;
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
impl<'a> Serializer for &mut fmt::Formatter<'a> {
    type Ok = ();
    type Error = fmt::Error;
    type SerializeSeq = Impossible<(), fmt::Error>;
    type SerializeTuple = Impossible<(), fmt::Error>;
    type SerializeTupleStruct = Impossible<(), fmt::Error>;
    type SerializeTupleVariant = Impossible<(), fmt::Error>;
    type SerializeMap = Impossible<(), fmt::Error>;
    type SerializeStruct = Impossible<(), fmt::Error>;
    type SerializeStructVariant = Impossible<(), fmt::Error>;
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> fmt::Result {}
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> fmt::Result
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_bytes(self, _v: &[u8]) -> fmt::Result {
        Err(fmt::Error)
    }
    fn serialize_none(self) -> fmt::Result {}
    fn serialize_some<T>(self, _value: &T) -> fmt::Result
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_unit(self) -> fmt::Result {}
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> fmt::Result
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, fmt::Error> {
        Err(fmt::Error)
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, fmt::Error> {
        Err(fmt::Error)
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, fmt::Error> {
        Err(fmt::Error)
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, fmt::Error> {
        Err(fmt::Error)
    }
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, fmt::Error> {
        Err(fmt::Error)
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, fmt::Error> {
        Err(fmt::Error)
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, fmt::Error> {
        Err(fmt::Error)
    }
    fn collect_str<T>(self, value: &T) -> fmt::Result
    where
        T: ?Sized + Display,
    {}
}
