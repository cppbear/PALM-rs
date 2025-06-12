use crate::lib::*;
use crate::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};
#[cfg(any(feature = "std", feature = "alloc"))]
use self::content::{
    Content, ContentSerializer, SerializeStructVariantAsMapValue,
    SerializeTupleVariantAsMapValue,
};
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
pub trait SerializeStruct {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
pub trait SerializeMap {
    type Ok;
    type Error: Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        tri!(self.serialize_key(key));
        self.serialize_value(value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
struct TaggedSerializer<S> {
    type_ident: &'static str,
    variant_ident: &'static str,
    tag: &'static str,
    variant_name: &'static str,
    delegate: S,
}
#[derive(Debug)]
pub struct Error;
pub struct SerializeMap<E> {
    entries: Vec<(Content, Content)>,
    key: Option<Content>,
    error: PhantomData<E>,
}
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
pub struct Impossible<Ok, Error> {
    void: Void,
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}
pub struct SerializeStruct<E> {
    name: &'static str,
    fields: Vec<(&'static str, Content)>,
    error: PhantomData<E>,
}
pub struct SerializeStructVariantAsMapValue<M> {
    map: M,
    name: &'static str,
    fields: Vec<(&'static str, Content)>,
}
pub struct SerializeTupleVariantAsMapValue<M> {
    map: M,
    name: &'static str,
    fields: Vec<Content>,
}
impl<S> Serializer for TaggedSerializer<S>
where
    S: Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = Impossible<S::Ok, S::Error>;
    type SerializeTuple = Impossible<S::Ok, S::Error>;
    type SerializeTupleStruct = Impossible<S::Ok, S::Error>;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    type SerializeTupleVariant = Impossible<S::Ok, S::Error>;
    #[cfg(any(feature = "std", feature = "alloc"))]
    type SerializeTupleVariant = SerializeTupleVariantAsMapValue<S::SerializeMap>;
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    type SerializeStructVariant = Impossible<S::Ok, S::Error>;
    #[cfg(any(feature = "std", feature = "alloc"))]
    type SerializeStructVariant = SerializeStructVariantAsMapValue<S::SerializeMap>;
    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Boolean))
    }
    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }
    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }
    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Char))
    }
    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::String))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::ByteArray))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Optional))
    }
    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(self.bad_type(Unsupported::Optional))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        map.end()
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        map.end()
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_entry(inner_variant, & ()));
        map.end()
    }
    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        inner_value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_entry(inner_variant, inner_value));
        map.end()
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(self.bad_type(Unsupported::Sequence))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(self.bad_type(Unsupported::Tuple))
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(self.bad_type(Unsupported::TupleStruct))
    }
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(self.bad_type(Unsupported::Enum))
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeTupleVariantAsMapValue::new(map, inner_variant, len))
    }
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(len.map(| len | len + 1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        Ok(map)
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let mut state = tri!(self.delegate.serialize_struct(name, len + 1));
        tri!(state.serialize_field(self.tag, self.variant_name));
        Ok(state)
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeStructVariantAsMapValue::new(map, inner_variant, len))
    }
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeStructVariantAsMapValue::new(map, inner_variant, len))
    }
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    fn collect_str<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        Err(self.bad_type(Unsupported::String))
    }
}
