use crate::lib::*;
use crate::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};
#[cfg(any(feature = "std", feature = "alloc"))]
use self::content::{
    Content, ContentSerializer, SerializeStructVariantAsMapValue,
    SerializeTupleVariantAsMapValue,
};
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
#[cfg(any(feature = "std", feature = "alloc"))]
pub struct FlatMapSerializeStruct<'a, M: 'a>(&'a mut M);
#[derive(Clone, PartialEq)]
pub struct Error {
    err: ErrorImpl,
}
#[derive(Debug)]
pub struct Error;
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a, M> ser::SerializeStruct for FlatMapSerializeStruct<'a, M>
where
    M: SerializeMap + 'a,
{
    type Ok = ();
    type Error = M::Error;
    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.serialize_entry(key, value)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
