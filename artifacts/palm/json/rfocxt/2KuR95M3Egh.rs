use crate::error::{Error, ErrorCode, Result};
use crate::map::Map;
use crate::value::{to_value, Value};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use core::result;
use serde::ser::{Impossible, Serialize};
struct MapKeySerializer;
pub struct Error;
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
impl serde::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;
    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {}
    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_bool(self, value: bool) -> Result<String> {}
    fn serialize_i8(self, value: i8) -> Result<String> {}
    fn serialize_i16(self, value: i16) -> Result<String> {}
    fn serialize_i32(self, value: i32) -> Result<String> {}
    fn serialize_i64(self, value: i64) -> Result<String> {}
    fn serialize_i128(self, value: i128) -> Result<String> {}
    fn serialize_u8(self, value: u8) -> Result<String> {}
    fn serialize_u16(self, value: u16) -> Result<String> {}
    fn serialize_u32(self, value: u32) -> Result<String> {}
    fn serialize_u64(self, value: u64) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }
    fn serialize_u128(self, value: u128) -> Result<String> {}
    fn serialize_f32(self, value: f32) -> Result<String> {}
    fn serialize_f64(self, value: f64) -> Result<String> {}
    #[inline]
    fn serialize_char(self, value: char) -> Result<String> {}
    #[inline]
    fn serialize_str(self, value: &str) -> Result<String> {}
    fn serialize_bytes(self, _value: &[u8]) -> Result<String> {}
    fn serialize_unit(self) -> Result<String> {}
    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {}
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_none(self) -> Result<String> {}
    fn serialize_some<T>(self, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }
    fn collect_str<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Display,
    {}
}
