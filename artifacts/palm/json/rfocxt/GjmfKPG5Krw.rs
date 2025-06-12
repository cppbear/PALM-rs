use crate::error::{Error, ErrorCode, Result};
use crate::map::Map;
use crate::value::{to_value, Value};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use core::result;
use serde::ser::{Impossible, Serialize};
pub struct Serializer;
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub struct SerializeVec {
    vec: Vec<Value>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
pub struct Error;
pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<Value>,
}
pub struct SerializeStructVariant {
    name: String,
    map: Map<String, Value>,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Value {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),
    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),
    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),
    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<Value>),
    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(Map<String, Value>),
}
pub enum SerializeMap {
    Map { map: Map<String, Value>, next_key: Option<String> },
    #[cfg(feature = "arbitrary_precision")]
    Number { out_value: Option<Value> },
    #[cfg(feature = "raw_value")]
    RawValue { out_value: Option<Value> },
}
impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = SerializeStructVariant;
    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Value> {}
    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value> {}
    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value> {}
    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value> {}
    fn serialize_i64(self, value: i64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }
    fn serialize_i128(self, value: i128) -> Result<Value> {}
    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Value> {}
    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Value> {}
    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Value> {}
    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Value> {}
    fn serialize_u128(self, value: u128) -> Result<Value> {}
    #[inline]
    fn serialize_f32(self, float: f32) -> Result<Value> {}
    #[inline]
    fn serialize_f64(self, float: f64) -> Result<Value> {}
    #[inline]
    fn serialize_char(self, value: char) -> Result<Value> {}
    #[inline]
    fn serialize_str(self, value: &str) -> Result<Value> {}
    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {}
    #[inline]
    fn serialize_unit(self) -> Result<Value> {}
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {}
    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {}
    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {}
    #[inline]
    fn serialize_none(self) -> Result<Value> {}
    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {}
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::Map {
            map: Map::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => {
                Ok(SerializeMap::Number {
                    out_value: None,
                })
            }
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => {
                Ok(SerializeMap::RawValue {
                    out_value: None,
                })
            }
            _ => self.serialize_map(Some(len)),
        }
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: String::from(variant),
            map: Map::new(),
        })
    }
    fn collect_str<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Display,
    {}
}
