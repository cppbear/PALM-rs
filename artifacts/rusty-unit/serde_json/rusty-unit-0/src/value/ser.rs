use crate::error::{Error, ErrorCode, Result};
use crate::map::Map;
use crate::value::{to_value, Value};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use core::result;
use serde::ser::{Impossible, Serialize};

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(v) => v.serialize(serializer),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = tri!(serializer.serialize_map(Some(m.len())));
                for (k, v) in m {
                    tri!(map.serialize_entry(k, v));
                }
                map.end()
            }
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Value::Object(_) => unreachable!(),
        }
    }
}

/// Serializer whose output is a `Value`.
///
/// This is the serializer that backs [`serde_json::to_value`][crate::to_value].
/// Unlike the main serde_json serializer which goes from some serializable
/// value of type `T` to JSON text, this one goes from `T` to
/// `serde_json::Value`.
///
/// The `to_value` function is implementable as:
///
/// ```
/// use serde::Serialize;
/// use serde_json::{Error, Value};
///
/// pub fn to_value<T>(input: T) -> Result<Value, Error>
/// where
///     T: Serialize,
/// {
///     input.serialize(serde_json::value::Serializer)
/// }
/// ```
pub struct Serializer;

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
    fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::Bool(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_i128(self, value: i128) -> Result<Value> {
        #[cfg(feature = "arbitrary_precision")]
        {
            Ok(Value::Number(value.into()))
        }

        #[cfg(not(feature = "arbitrary_precision"))]
        {
            if let Ok(value) = u64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else if let Ok(value) = i64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
            }
        }
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Value> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u128(self, value: u128) -> Result<Value> {
        #[cfg(feature = "arbitrary_precision")]
        {
            Ok(Value::Number(value.into()))
        }

        #[cfg(not(feature = "arbitrary_precision"))]
        {
            if let Ok(value) = u64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(Error::syntax(ErrorCode::NumberOutOfRange, 0, 0))
            }
        }
    }

    #[inline]
    fn serialize_f32(self, float: f32) -> Result<Value> {
        Ok(Value::from(float))
    }

    #[inline]
    fn serialize_f64(self, float: f64) -> Result<Value> {
        Ok(Value::from(float))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Value> {
        let mut s = String::new();
        s.push(value);
        Ok(Value::String(s))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        let mut values = Map::new();
        values.insert(String::from(variant), tri!(to_value(value)));
        Ok(Value::Object(values))
    }

    #[inline]
    fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

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

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match name {
            #[cfg(feature = "arbitrary_precision")]
            crate::number::TOKEN => Ok(SerializeMap::Number { out_value: None }),
            #[cfg(feature = "raw_value")]
            crate::raw::TOKEN => Ok(SerializeMap::RawValue { out_value: None }),
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
    {
        Ok(Value::String(value.to_string()))
    }
}

pub struct SerializeVec {
    vec: Vec<Value>,
}

pub struct SerializeTupleVariant {
    name: String,
    vec: Vec<Value>,
}

pub enum SerializeMap {
    Map {
        map: Map<String, Value>,
        next_key: Option<String>,
    },
    #[cfg(feature = "arbitrary_precision")]
    Number { out_value: Option<Value> },
    #[cfg(feature = "raw_value")]
    RawValue { out_value: Option<Value> },
}

pub struct SerializeStructVariant {
    name: String,
    map: Map<String, Value>,
}

impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(tri!(to_value(value)));
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Array(self.vec))
    }
}

impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(tri!(to_value(value)));
        Ok(())
    }

    fn end(self) -> Result<Value> {
        let mut object = Map::new();

        object.insert(self.name, Value::Array(self.vec));

        Ok(Value::Object(object))
    }
}

impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { next_key, .. } => {
                *next_key = Some(tri!(key.serialize(MapKeySerializer)));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { map, next_key } => {
                let key = next_key.take();
                // Panic because this indicates a bug in the program rather than an
                // expected failure.
                let key = key.expect("serialize_value called before serialize_key");
                map.insert(key, tri!(to_value(value)));
                Ok(())
            }
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }

    fn end(self) -> Result<Value> {
        match self {
            SerializeMap::Map { map, .. } => Ok(Value::Object(map)),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { .. } => unreachable!(),
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { .. } => unreachable!(),
        }
    }
}

struct MapKeySerializer;

fn key_must_be_a_string() -> Error {
    Error::syntax(ErrorCode::KeyMustBeAString, 0, 0)
}

fn float_key_must_be_finite() -> Error {
    Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0)
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
    ) -> Result<String> {
        Ok(variant.to_owned())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_bool(self, value: bool) -> Result<String> {
        Ok(if value { "true" } else { "false" }.to_owned())
    }

    fn serialize_i8(self, value: i8) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_i16(self, value: i16) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_i32(self, value: i32) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_i64(self, value: i64) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_i128(self, value: i128) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_u8(self, value: u8) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_u16(self, value: u16) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_u32(self, value: u32) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_u64(self, value: u64) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_u128(self, value: u128) -> Result<String> {
        Ok(itoa::Buffer::new().format(value).to_owned())
    }

    fn serialize_f32(self, value: f32) -> Result<String> {
        if value.is_finite() {
            Ok(ryu::Buffer::new().format_finite(value).to_owned())
        } else {
            Err(float_key_must_be_finite())
        }
    }

    fn serialize_f64(self, value: f64) -> Result<String> {
        if value.is_finite() {
            Ok(ryu::Buffer::new().format_finite(value).to_owned())
        } else {
            Err(float_key_must_be_finite())
        }
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<String> {
        Ok({
            let mut s = String::new();
            s.push(value);
            s
        })
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<String> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

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

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
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
    {
        Ok(value.to_string())
    }
}

impl serde::ser::SerializeStruct for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            SerializeMap::Map { .. } => serde::ser::SerializeMap::serialize_entry(self, key, value),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { out_value } => {
                if key == crate::number::TOKEN {
                    *out_value = Some(tri!(value.serialize(NumberValueEmitter)));
                    Ok(())
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { out_value } => {
                if key == crate::raw::TOKEN {
                    *out_value = Some(tri!(value.serialize(RawValueEmitter)));
                    Ok(())
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }

    fn end(self) -> Result<Value> {
        match self {
            SerializeMap::Map { .. } => serde::ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            SerializeMap::Number { out_value, .. } => {
                Ok(out_value.expect("number value was not emitted"))
            }
            #[cfg(feature = "raw_value")]
            SerializeMap::RawValue { out_value, .. } => {
                Ok(out_value.expect("raw value was not emitted"))
            }
        }
    }
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map.insert(String::from(key), tri!(to_value(value)));
        Ok(())
    }

    fn end(self) -> Result<Value> {
        let mut object = Map::new();

        object.insert(self.name, Value::Object(self.map));

        Ok(Value::Object(object))
    }
}

#[cfg(feature = "arbitrary_precision")]
struct NumberValueEmitter;

#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
    Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}

#[cfg(feature = "arbitrary_precision")]
impl serde::ser::Serializer for NumberValueEmitter {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = Impossible<Value, Error>;
    type SerializeTuple = Impossible<Value, Error>;
    type SerializeTupleStruct = Impossible<Value, Error>;
    type SerializeTupleVariant = Impossible<Value, Error>;
    type SerializeMap = Impossible<Value, Error>;
    type SerializeStruct = Impossible<Value, Error>;
    type SerializeStructVariant = Impossible<Value, Error>;

    fn serialize_bool(self, _v: bool) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_i8(self, _v: i8) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_i16(self, _v: i16) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_i32(self, _v: i32) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_i64(self, _v: i64) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_u8(self, _v: u8) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_u16(self, _v: u16) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_u32(self, _v: u32) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_u64(self, _v: u64) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_f32(self, _v: f32) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_f64(self, _v: f64) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_char(self, _v: char) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_str(self, value: &str) -> Result<Value> {
        let n = tri!(value.to_owned().parse());
        Ok(Value::Number(n))
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_none(self) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }

    fn serialize_unit(self) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Value> {
        Err(invalid_number())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(invalid_number())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(invalid_number())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(invalid_number())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(invalid_number())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(invalid_number())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(invalid_number())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(invalid_number())
    }
}

#[cfg(feature = "raw_value")]
struct RawValueEmitter;

#[cfg(feature = "raw_value")]
fn invalid_raw_value() -> Error {
    Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0)
}

#[cfg(feature = "raw_value")]
impl serde::ser::Serializer for RawValueEmitter {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = Impossible<Value, Error>;
    type SerializeTuple = Impossible<Value, Error>;
    type SerializeTupleStruct = Impossible<Value, Error>;
    type SerializeTupleVariant = Impossible<Value, Error>;
    type SerializeMap = Impossible<Value, Error>;
    type SerializeStruct = Impossible<Value, Error>;
    type SerializeStructVariant = Impossible<Value, Error>;

    fn serialize_bool(self, _v: bool) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_i8(self, _v: i8) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_i16(self, _v: i16) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_i32(self, _v: i32) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_i64(self, _v: i64) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_u8(self, _v: u8) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_u16(self, _v: u16) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_u32(self, _v: u32) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_u64(self, _v: u64) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_f32(self, _v: f32) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_f64(self, _v: f64) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_char(self, _v: char) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_str(self, value: &str) -> Result<Value> {
        crate::from_str(value)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_none(self) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_raw_value())
    }

    fn serialize_unit(self) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Value> {
        Err(invalid_raw_value())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_raw_value())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_raw_value())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(invalid_raw_value())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(invalid_raw_value())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(invalid_raw_value())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(invalid_raw_value())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(invalid_raw_value())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(invalid_raw_value())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(invalid_raw_value())
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        self.serialize_str(&value.to_string())
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::ExactSizeIterator;
	use std::iter::FromIterator;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::error::Error;
	use std::iter::Iterator;
	use read::Read;
	use std::iter::DoubleEndedIterator;
	use std::cmp::Eq;
	use std::ops::Deref;
// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_5() {
//     rusty_monitor::set_test_id(5);
//     let mut bool_0: bool = true;
//     let mut value_0: value::Value = crate::value::Value::Bool(bool_0);
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut state_0: ser::State = crate::ser::State::Rest;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut u128_0: u128 = 2450u128;
//     let mut value_1: value::Value = crate::value::Value::Null;
//     let mut str_0: &str = "2fvPmh7ana3tCHw4h";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut error_0: crate::error::Error = crate::value::ser::key_must_be_a_string();
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut state_1: ser::State = crate::ser::State::Empty;
//     let mut state_1_ref_0: &ser::State = &mut state_1;
//     let mut f64_0: f64 = -678.760275f64;
//     let mut usize_0: usize = 612usize;
//     let mut usize_1: usize = 8788usize;
//     let mut error_1: crate::error::Error = crate::value::ser::key_must_be_a_string();
//     let mut error_1_ref_0: &crate::error::Error = &mut error_1;
//     let mut error_2: crate::error::Error = crate::value::ser::float_key_must_be_finite();
//     let mut error_2_ref_0: &crate::error::Error = &mut error_2;
//     let mut option_0: std::option::Option<std::io::ErrorKind> = crate::error::Error::io_error_kind(error_2_ref_0);
//     let mut state_2: ser::State = crate::ser::State::Rest;
//     let mut bool_1: bool = crate::error::Error::is_data(error_1_ref_0);
//     let mut position_0: crate::read::Position = crate::read::Position {line: usize_1, column: usize_0};
//     let mut option_1: std::option::Option<crate::number::Number> = crate::number::Number::from_f64(f64_0);
//     let mut tuple_0: () = crate::ser::State::assert_receiver_is_total_eq(state_1_ref_0);
//     let mut option_2: std::option::Option<&dyn std::error::Error> = crate::error::Error::source(error_0_ref_0);
//     crate::de::from_str(str_0_ref_0);
//     let mut errorkind_0: std::io::ErrorKind = std::option::Option::unwrap(option_0);
//     let mut option_3: std::option::Option<crate::number::Number> = crate::number::Number::from_u128(u128_0);
//     let mut tuple_1: () = crate::ser::State::assert_receiver_is_total_eq(state_0_ref_0);
//     let mut number_0: crate::number::Number = std::option::Option::unwrap(option_3);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_6() {
//     rusty_monitor::set_test_id(6);
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::iter_mut(map_0_ref_0);
//     let mut itermut_0_ref_0: &crate::map::IterMut = &mut itermut_0;
//     let mut state_0: ser::State = crate::ser::State::Rest;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_2_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut itermut_1: crate::map::IterMut = crate::map::Map::into_iter(map_2_ref_0);
//     let mut itermut_1_ref_0: &mut crate::map::IterMut = &mut itermut_1;
//     let mut i128_0: i128 = 1787i128;
//     let mut str_0: &str = "lvAq4jQ8";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &crate::read::StrRead = &mut strread_0;
//     let mut error_0: crate::error::Error = crate::value::ser::float_key_must_be_finite();
//     let mut position_0: crate::read::Position = crate::read::StrRead::peek_position(strread_0_ref_0);
//     let mut option_0: std::option::Option<crate::number::Number> = crate::number::Number::from_i128(i128_0);
//     let mut state_1: ser::State = crate::ser::State::Rest;
//     let mut number_0: crate::number::Number = std::option::Option::unwrap(option_0);
//     let mut option_1: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next(itermut_1_ref_0);
//     let mut usize_0: usize = crate::map::Map::len(map_1_ref_0);
//     let mut category_0: error::Category = crate::error::Category::Data;
//     let mut state_1_ref_0: &ser::State = &mut state_1;
//     let mut bool_0: bool = crate::ser::State::eq(state_1_ref_0, state_0_ref_0);
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut usize_1: usize = crate::error::Error::line(error_0_ref_0);
//     let mut usize_2: usize = crate::map::IterMut::len(itermut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut usize_0: usize = 8544usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_0_ref_0);
    let mut keys_0_ref_0: &mut crate::map::Keys = &mut keys_0;
    let mut i128_0: i128 = 17227i128;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut values_0: crate::map::Values = crate::map::Map::values(map_1_ref_0);
    let mut values_0_ref_0: &mut crate::map::Values = &mut values_0;
    let mut category_0: error::Category = crate::error::Category::Syntax;
    let mut category_0_ref_0: &error::Category = &mut category_0;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut error_0: crate::error::Error = crate::value::ser::key_must_be_a_string();
    let mut error_0_ref_0: &crate::error::Error = &mut error_0;
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_3_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_3;
    let mut keys_1: crate::map::Keys = crate::map::Map::keys(map_3_ref_0);
    let mut keys_1_ref_0: &mut crate::map::Keys = &mut keys_1;
    let mut str_0: &str = "Ul0bm72Mk";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    crate::read::StrRead::peek(strread_0_ref_0);
    let mut option_0: std::option::Option<&std::string::String> = crate::map::Keys::next_back(keys_1_ref_0);
    let mut usize_1: usize = crate::error::Error::column(error_0_ref_0);
    let mut string_0: &std::string::String = std::option::Option::unwrap(option_0);
    let mut state_0: ser::State = crate::ser::State::Rest;
    let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::clone(map_2_ref_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::Solidus;
    let mut map_4_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_4;
    let mut option_1: std::option::Option<&value::Value> = crate::map::Values::next(values_0_ref_0);
    let mut option_2: std::option::Option<crate::number::Number> = crate::number::Number::from_i128(i128_0);
    let mut option_3: std::option::Option<&std::string::String> = crate::map::Keys::next(keys_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_25() {
//     rusty_monitor::set_test_id(25);
//     let mut str_0: &str = "8b0yrUBvKRC5RvAALJm";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
//     let mut state_0: ser::State = crate::ser::State::Empty;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut isize_0: isize = 13191isize;
//     let mut isize_0_ref_0: &isize = &mut isize_0;
//     let mut usize_0: usize = 9439usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut usize_1: usize = 3794usize;
//     let mut usize_2: usize = 7078usize;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut values_0: crate::map::Values = crate::map::Map::values(map_1_ref_0);
//     let mut values_0_ref_0: &crate::map::Values = &mut values_0;
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Values::size_hint(values_0_ref_0);
//     let mut error_0: crate::error::Error = crate::value::ser::key_must_be_a_string();
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut bool_0: bool = crate::error::Error::is_eof(error_0_ref_0);
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::ReverseSolidus;
//     let mut valuesmut_0: crate::map::ValuesMut = crate::map::Map::values_mut(map_0_ref_0);
//     let mut reference_0: read::Reference<isize> = crate::read::Reference::Borrowed(isize_0_ref_0);
//     let mut reference_0_ref_0: &read::Reference<isize> = &mut reference_0;
//     let mut isize_1: &isize = crate::read::Reference::deref(reference_0_ref_0);
//     let mut tuple_1: () = crate::ser::State::assert_receiver_is_total_eq(state_0_ref_0);
//     crate::read::StrRead::discard(strread_0_ref_0);
//     let mut valuesmut_0_ref_0: &mut crate::map::ValuesMut = &mut valuesmut_0;
//     let mut option_0: std::option::Option<&mut value::Value> = crate::map::ValuesMut::next(valuesmut_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_26() {
//     rusty_monitor::set_test_id(26);
//     let mut usize_0: usize = 9163usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut values_0: crate::map::Values = crate::map::Map::values(map_1_ref_0);
//     let mut values_0_ref_0: &crate::map::Values = &mut values_0;
//     let mut usize_1: usize = 5514usize;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
//     let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut u8_0: u8 = 123u8;
//     let mut usize_2: usize = 146usize;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_2);
//     let mut map_3_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_3;
//     let mut iter_0: crate::map::Iter = crate::map::Map::into_iter(map_3_ref_0);
//     let mut iter_0_ref_0: &mut crate::map::Iter = &mut iter_0;
//     let mut error_0: crate::error::Error = crate::value::ser::float_key_must_be_finite();
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut i64_0: i64 = -11353i64;
//     let mut usize_3: usize = crate::error::Error::line(error_0_ref_0);
//     let mut state_0: ser::State = crate::ser::State::First;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut tuple_0: () = crate::ser::State::assert_receiver_is_total_eq(state_0_ref_0);
//     let mut option_0: std::option::Option<(&std::string::String, &value::Value)> = crate::map::Iter::next(iter_0_ref_0);
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::AsciiControl(u8_0);
//     let mut tuple_1: (&std::string::String, &value::Value) = std::option::Option::unwrap(option_0);
//     let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_2_ref_0);
//     let mut keys_0_ref_0: &mut crate::map::Keys = &mut keys_0;
//     let mut option_1: std::option::Option<&std::string::String> = crate::map::Keys::next_back(keys_0_ref_0);
//     let mut tuple_2: (usize, std::option::Option<usize>) = crate::map::Values::size_hint(values_0_ref_0);
//     let mut charescape_1: ser::CharEscape = crate::ser::CharEscape::CarriageReturn;
//     let mut usize_4: usize = crate::map::Map::len(map_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_28() {
//     rusty_monitor::set_test_id(28);
//     let mut str_0: &str = "T4";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut value_0: value::Value = crate::value::Value::Object(map_0);
//     let mut error_0: crate::error::Error = crate::value::ser::float_key_must_be_finite();
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut error_1: crate::error::Error = crate::value::ser::key_must_be_a_string();
//     let mut error_1_ref_0: &crate::error::Error = &mut error_1;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut usize_0: usize = 1975usize;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_3);
//     let mut intoiter_0_ref_0: &crate::map::IntoIter = &mut intoiter_0;
//     let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut error_2: crate::error::Error = crate::value::ser::float_key_must_be_finite();
//     let mut error_2_ref_0: &crate::error::Error = &mut error_2;
//     let mut u8_0: u8 = 42u8;
//     let mut isize_0: isize = -5229isize;
//     let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_4);
//     let mut usize_1: usize = crate::map::IntoIter::len(intoiter_0_ref_0);
//     let mut option_0: std::option::Option<&dyn std::error::Error> = crate::error::Error::source(error_1_ref_0);
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::into_iter(map_2_ref_0);
//     let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_1_ref_0);
//     let mut option_1: std::option::Option<std::io::ErrorKind> = crate::error::Error::io_error_kind(error_0_ref_0);
//     let mut intovalues_0_ref_0: &mut crate::map::IntoValues = &mut intovalues_0;
//     let mut option_2: std::option::Option<value::Value> = crate::map::IntoValues::next_back(intovalues_0_ref_0);
//     let mut error_3: &dyn std::error::Error = std::option::Option::unwrap(option_0);
//     crate::read::StrRead::next(strread_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut usize_0: usize = 2643usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut usize_1: usize = 6730usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_1);
    let mut intoiter_0_ref_0: &crate::map::IntoIter = &mut intoiter_0;
    let mut str_0: &str = "CZ3nG6jvc2rfgA";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_2_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut valuesmut_0: crate::map::ValuesMut = crate::map::Map::values_mut(map_2_ref_0);
    let mut valuesmut_0_ref_0: &crate::map::ValuesMut = &mut valuesmut_0;
    let mut usize_2: usize = 6065usize;
    let mut str_1: &str = "9BNwF9M4pcvj02";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
    let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
    let mut error_0: crate::error::Error = crate::value::ser::key_must_be_a_string();
    let mut error_0_ref_0: &crate::error::Error = &mut error_0;
    let mut usize_3: usize = crate::error::Error::line(error_0_ref_0);
    crate::read::StrRead::peek(strread_1_ref_0);
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_2);
    let mut usize_4: usize = crate::map::ValuesMut::len(valuesmut_0_ref_0);
    crate::read::StrRead::peek(strread_0_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::IntoIter::size_hint(intoiter_0_ref_0);
    let mut map_3_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_3;
    crate::map::Map::append(map_3_ref_0, map_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u8_0: u8 = 81u8;
    let mut usize_0: usize = 8820usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut error_0: crate::error::Error = crate::value::ser::float_key_must_be_finite();
    let mut error_0_ref_0: &crate::error::Error = &mut error_0;
    let mut error_1: crate::error::Error = crate::value::ser::float_key_must_be_finite();
    let mut error_1_ref_0: &crate::error::Error = &mut error_1;
    let mut error_2: crate::error::Error = crate::value::ser::key_must_be_a_string();
    let mut error_2_ref_0: &crate::error::Error = &mut error_2;
    let mut usize_1: usize = 284usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut valuesmut_0: crate::map::ValuesMut = crate::map::Map::values_mut(map_1_ref_0);
    let mut valuesmut_0_ref_0: &crate::map::ValuesMut = &mut valuesmut_0;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
    let mut str_0: &str = "lVuKD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut usize_2: usize = 1765usize;
    crate::read::StrRead::parse_str(strread_0_ref_0, vec_0_ref_0);
    let mut usize_3: usize = crate::map::ValuesMut::len(valuesmut_0_ref_0);
    let mut bool_0: bool = crate::error::Error::is_data(error_2_ref_0);
    let mut bool_1: bool = crate::error::Error::is_data(error_1_ref_0);
    let mut bool_2: bool = crate::error::Error::is_eof(error_0_ref_0);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::AsciiControl(u8_0);
    let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
    let mut option_0: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}
}