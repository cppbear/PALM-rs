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
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}
pub trait Read<'de>: private::Sealed {
    const should_early_return_if_failed: bool;
    fn next(&mut self) -> Result<Option<u8>>;
    fn peek(&mut self) -> Result<Option<u8>>;
    fn discard(&mut self);
    fn position(&self) -> Position;
    fn peek_position(&self) -> Position;
    fn byte_offset(&self) -> usize;
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>>;
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;
    fn ignore_str(&mut self) -> Result<()>;
    fn decode_hex_escape(&mut self) -> Result<u16>;
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self);
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;
    fn set_failed(&mut self, failed: &mut bool);
}
pub(crate) trait FloatErrors {
    fn error_scale() -> u32;
    fn error_halfscale() -> u32;
    fn error_is_accurate<F: Float>(count: u32, fp: &ExtendedFloat) -> bool;
}
pub trait Float: Number {
    type Unsigned: Integer;
    const ZERO: Self;
    const MAX_DIGITS: usize;
    const EXPONENT_MASK: Self::Unsigned;
    const HIDDEN_BIT_MASK: Self::Unsigned;
    const MANTISSA_MASK: Self::Unsigned;
    const INFINITY_BITS: Self::Unsigned;
    const MANTISSA_SIZE: i32;
    const EXPONENT_BIAS: i32;
    const DENORMAL_EXPONENT: i32;
    const MAX_EXPONENT: i32;
    const DEFAULT_SHIFT: i32;
    const CARRY_MASK: u64;
    fn exponent_limit() -> (i32, i32);
    fn mantissa_limit() -> i32;
    fn pow10(self, n: i32) -> Self;
    fn from_bits(u: Self::Unsigned) -> Self;
    fn to_bits(self) -> Self::Unsigned;
    fn is_sign_positive(self) -> bool;
    #[inline]
    fn is_denormal(self) -> bool;
    #[inline]
    fn is_special(self) -> bool;
    #[inline]
    fn is_inf(self) -> bool;
    #[inline]
    fn exponent(self) -> i32;
    #[inline]
    fn mantissa(self) -> Self::Unsigned {
        let bits = self.to_bits();
        let s = bits & Self::MANTISSA_MASK;
        if !self.is_denormal() { s + Self::HIDDEN_BIT_MASK } else { s }
    }
    #[inline]
    fn next_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && ! self.is_inf());
        Self::from_bits(self.to_bits() + Self::Unsigned::as_cast(1u32))
    }
    #[inline]
    fn round_positive_even(self) -> Self {
        if self.mantissa() & Self::Unsigned::as_cast(1u32)
            == Self::Unsigned::as_cast(1u32)
        {
            self.next_positive()
        } else {
            self
        }
    }
}
trait Hi64<T>: AsRef<[T]> {
    fn hi64_1(&self) -> (u64, bool);
    fn hi64_2(&self) -> (u64, bool);
    fn hi64_3(&self) -> (u64, bool);
    #[inline]
    fn hi64(&self) -> (u64, bool);
}
pub trait Index: private::Sealed {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}
pub struct Serializer;
pub struct Error;
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub struct Deserializer<R> {
    read: R,
    scratch: Vec<u8>,
    remaining_depth: u8,
    #[cfg(feature = "float_roundtrip")]
    single_precision: bool,
    #[cfg(feature = "unbounded_depth")]
    disable_recursion_limit: bool,
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
impl<'de> serde::Deserializer<'de> for &'de Value {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {}
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
    {
        self.deserialize_unit(visitor)
    }
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
