use crate::lib::*;
use crate::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};
#[cfg(any(feature = "std", feature = "alloc"))]
use self::content::{
    Content, ContentSerializer, SerializeStructVariantAsMapValue,
    SerializeTupleVariantAsMapValue,
};
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
#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializeMap<'a, M: 'a>(&'a mut M);
#[derive(Debug)]
pub struct Error;
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
pub struct T;
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> ser::SerializeMap for FlatMapSerializeMap<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_key(key)
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_value(value)
    }
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.0.serialize_entry(key, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
