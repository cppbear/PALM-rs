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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
pub struct Error;
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
impl<'de> serde::Deserializer<'de> for Value {
    type Error = Error;
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Number(n) => n.deserialize_any(visitor),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Value::String(_) => unreachable!(),
            Value::Array(v) => visit_array(v, visitor),
            Value::Object(v) => v.deserialize_any(visitor),
        }
    }
    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    #[inline]
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
}
fn visit_array<'de, V>(array: Vec<Value>, visitor: V) -> Result<V::Value, Error>
where
    V: Visitor<'de>,
{
    let len = array.len();
    let mut deserializer = SeqDeserializer::new(array);
    let seq = tri!(visitor.visit_seq(& mut deserializer));
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(len, &"fewer elements in array"))
    }
}
