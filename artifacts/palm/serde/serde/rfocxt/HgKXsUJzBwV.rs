use crate::lib::*;
use crate::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
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
pub trait SerializeTuple {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
pub trait SerializeTupleVariant {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
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
pub trait SerializeSeq {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
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
enum Void {}
impl<Ok, Error> SerializeMap for Impossible<Ok, Error>
where
    Error: ser::Error,
{
    type Ok = Ok;
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {}
    fn end(self) -> Result<Ok, Error> {
        match self.void {}
    }
}
