use crate::error::{Error, ErrorCode};
use crate::map::Map;
use crate::number::Number;
use crate::value::Value;
use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
#[cfg(feature = "raw_value")]
use alloc::string::ToString;
use alloc::vec::{self, Vec};
use core::fmt;
use core::slice;
use core::str::FromStr;
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer as _, EnumAccess, Expected,
    IntoDeserializer, MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;
#[cfg(feature = "arbitrary_precision")]
use crate::number::NumberFromString;
macro_rules! deserialize_number {
    ($method:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn $method < V > (self, visitor : V)
        -> Result < V::Value, Error > where V : Visitor <'de >, { match self {
        Value::Number(n) => n.deserialize_any(visitor), _ => Err(self.invalid_type(&
        visitor)), } } #[cfg(feature = "arbitrary_precision")] fn $method < V > (self,
        visitor : V) -> Result < V::Value, Error > where V : Visitor <'de >, { match self
        { Value::Number(n) => n.$method (visitor), _ => self.deserialize_any(visitor), }
        }
    };
}
macro_rules! deserialize_value_ref_number {
    ($method:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))] fn $method < V > (self, visitor : V)
        -> Result < V::Value, Error > where V : Visitor <'de >, { match self {
        Value::Number(n) => n.deserialize_any(visitor), _ => Err(self.invalid_type(&
        visitor)), } } #[cfg(feature = "arbitrary_precision")] fn $method < V > (self,
        visitor : V) -> Result < V::Value, Error > where V : Visitor <'de >, { match self
        { Value::Number(n) => n.$method (visitor), _ => self.deserialize_any(visitor), }
        }
    };
}
macro_rules! deserialize_numeric_key {
    ($method:ident) => {
        deserialize_numeric_key!($method, deserialize_number);
    };
    ($method:ident, $using:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value, Error > where V :
        Visitor <'de >, { let mut de = crate ::Deserializer::from_str(& self.key); match
        tri!(de.peek()) { Some(b'0'..= b'9' | b'-') => {} _ => return
        Err(Error::syntax(ErrorCode::ExpectedNumericKey, 0, 0)), } let number = tri!(de
        .$using (visitor)); if tri!(de.peek()) .is_some() { return
        Err(Error::syntax(ErrorCode::ExpectedNumericKey, 0, 0)); } Ok(number) }
    };
}
pub trait Number: AsCast + ops::Add<Output = Self> {}
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
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
impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }
            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Bool(value))
            }
            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }
            fn visit_i128<E>(self, value: i128) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                let de = serde::de::value::I128Deserializer::new(value);
                Number::deserialize(de).map(Value::Number)
            }
            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::Number(value.into()))
            }
            fn visit_u128<E>(self, value: u128) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                let de = serde::de::value::U128Deserializer::new(value);
                Number::deserialize(de).map(Value::Number)
            }
            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Number::from_f64(value).map_or(Value::Null, Value::Number))
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value))
            }
            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }
            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }
            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }
            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(elem) = tri!(visitor.next_element()) {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                match tri!(visitor.next_key_seed(KeyClassifier)) {
                    #[cfg(feature = "arbitrary_precision")]
                    Some(KeyClass::Number) => {
                        let number: NumberFromString = tri!(visitor.next_value());
                        Ok(Value::Number(number.value))
                    }
                    #[cfg(feature = "raw_value")]
                    Some(KeyClass::RawValue) => {
                        let value = tri!(
                            visitor.next_value_seed(crate ::raw::BoxedFromString)
                        );
                        crate::from_str(value.get()).map_err(de::Error::custom)
                    }
                    Some(KeyClass::Map(first_key)) => {
                        let mut values = Map::new();
                        values.insert(first_key, tri!(visitor.next_value()));
                        while let Some((key, value)) = tri!(visitor.next_entry()) {
                            values.insert(key, value);
                        }
                        Ok(Value::Object(values))
                    }
                    None => Ok(Value::Object(Map::new())),
                }
            }
        }
        deserializer.deserialize_any(ValueVisitor)
    }
}
