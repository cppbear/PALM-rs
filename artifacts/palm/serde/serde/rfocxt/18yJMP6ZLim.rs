use crate::lib::*;
use crate::ser::{self, Serialize, Serializer};
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
pub trait SerializeTupleVariant {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
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
pub trait SerializeStructVariant {
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
pub trait SerializeTuple {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
pub trait SerializeSeq {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
pub trait SerializeTupleStruct {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
pub struct ContentSerializer<E> {
    error: PhantomData<E>,
}
pub struct SerializeStructVariant<E> {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    fields: Vec<(&'static str, Content)>,
    error: PhantomData<E>,
}
pub struct SerializeTupleVariant<E> {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    fields: Vec<Content>,
    error: PhantomData<E>,
}
pub struct SerializeStruct<E> {
    name: &'static str,
    fields: Vec<(&'static str, Content)>,
    error: PhantomData<E>,
}
pub struct SerializeMap<E> {
    entries: Vec<(Content, Content)>,
    key: Option<Content>,
    error: PhantomData<E>,
}
pub struct SerializeTuple<E> {
    elements: Vec<Content>,
    error: PhantomData<E>,
}
pub struct SerializeSeq<E> {
    elements: Vec<Content>,
    error: PhantomData<E>,
}
pub struct SerializeTupleStruct<E> {
    name: &'static str,
    fields: Vec<Content>,
    error: PhantomData<E>,
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
impl<E> Serializer for ContentSerializer<E>
where
    E: ser::Error,
{
    type Ok = Content;
    type Error = E;
    type SerializeSeq = SerializeSeq<E>;
    type SerializeTuple = SerializeTuple<E>;
    type SerializeTupleStruct = SerializeTupleStruct<E>;
    type SerializeTupleVariant = SerializeTupleVariant<E>;
    type SerializeMap = SerializeMap<E>;
    type SerializeStruct = SerializeStruct<E>;
    type SerializeStructVariant = SerializeStructVariant<E>;
    fn serialize_bool(self, v: bool) -> Result<Content, E> {}
    fn serialize_i8(self, v: i8) -> Result<Content, E> {}
    fn serialize_i16(self, v: i16) -> Result<Content, E> {}
    fn serialize_i32(self, v: i32) -> Result<Content, E> {}
    fn serialize_i64(self, v: i64) -> Result<Content, E> {}
    fn serialize_u8(self, v: u8) -> Result<Content, E> {}
    fn serialize_u16(self, v: u16) -> Result<Content, E> {}
    fn serialize_u32(self, v: u32) -> Result<Content, E> {}
    fn serialize_u64(self, v: u64) -> Result<Content, E> {}
    fn serialize_f32(self, v: f32) -> Result<Content, E> {}
    fn serialize_f64(self, v: f64) -> Result<Content, E> {}
    fn serialize_char(self, v: char) -> Result<Content, E> {}
    fn serialize_str(self, value: &str) -> Result<Content, E> {}
    fn serialize_bytes(self, value: &[u8]) -> Result<Content, E> {}
    fn serialize_none(self) -> Result<Content, E> {}
    fn serialize_some<T>(self, value: &T) -> Result<Content, E>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_unit(self) -> Result<Content, E> {}
    fn serialize_unit_struct(self, name: &'static str) -> Result<Content, E> {}
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Content, E> {}
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Content, E>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Content, E>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, E> {
        Ok(SerializeSeq {
            elements: Vec::with_capacity(len.unwrap_or(0)),
            error: PhantomData,
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, E> {
        Ok(SerializeTuple {
            elements: Vec::with_capacity(len),
            error: PhantomData,
        })
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, E> {
        Ok(SerializeTupleStruct {
            name,
            fields: Vec::with_capacity(len),
            error: PhantomData,
        })
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, E> {
        Ok(SerializeTupleVariant {
            name,
            variant_index,
            variant,
            fields: Vec::with_capacity(len),
            error: PhantomData,
        })
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, E> {
        Ok(SerializeMap {
            entries: Vec::with_capacity(len.unwrap_or(0)),
            key: None,
            error: PhantomData,
        })
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, E> {
        Ok(SerializeStruct {
            name,
            fields: Vec::with_capacity(len),
            error: PhantomData,
        })
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, E> {
        Ok(SerializeStructVariant {
            name,
            variant_index,
            variant,
            fields: Vec::with_capacity(len),
            error: PhantomData,
        })
    }
}
