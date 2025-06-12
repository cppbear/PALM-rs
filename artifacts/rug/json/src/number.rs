use crate::de::ParserNumber;
use crate::error::Error;
#[cfg(feature = "arbitrary_precision")]
use crate::error::ErrorCode;
#[cfg(feature = "arbitrary_precision")]
use alloc::borrow::ToOwned;
#[cfg(feature = "arbitrary_precision")]
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
#[cfg(not(feature = "arbitrary_precision"))]
use core::hash::{Hash, Hasher};
use serde::de::{self, Unexpected, Visitor};
#[cfg(feature = "arbitrary_precision")]
use serde::de::{IntoDeserializer, MapAccess};
use serde::{forward_to_deserialize_any, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "arbitrary_precision")]
pub(crate) const TOKEN: &str = "$serde_json::private::Number";

/// Represents a JSON number, whether integer or floating point.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}

#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

#[cfg(not(feature = "arbitrary_precision"))]
impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}

// Implementing Eq is fine since any float values are always finite.
#[cfg(not(feature = "arbitrary_precision"))]
impl Eq for N {}

#[cfg(not(feature = "arbitrary_precision"))]
impl Hash for N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    // There are 2 zero representations, +0 and -0, which
                    // compare equal but have different bits. We use the +0 hash
                    // for both so that hash(+0) == hash(-0).
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}

#[cfg(feature = "arbitrary_precision")]
type N = String;

impl Number {
    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    pub fn is_i64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(v) => v <= i64::MAX as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.as_i64().is_some()
    }

    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    pub fn is_u64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(_) => true,
            N::NegInt(_) | N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.as_u64().is_some()
    }

    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    pub fn is_f64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::Float(_) => true,
            N::PosInt(_) | N::NegInt(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        {
            for c in self.n.chars() {
                if c == '.' || c == 'e' || c == 'E' {
                    return self.n.parse::<f64>().ok().map_or(false, f64::is_finite);
                }
            }
            false
        }
    }

    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }

    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n),
            N::NegInt(_) | N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f64>().ok().filter(|float| float.is_finite())
    }

    /// Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
    /// numbers.
    ///
    /// ```
    /// # use serde_json::Number;
    /// #
    /// assert!(Number::from_f64(256.0).is_some());
    ///
    /// assert!(Number::from_f64(f64::NAN).is_none());
    /// ```
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            let n = {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::Float(f)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    ryu::Buffer::new().format_finite(f).to_owned()
                }
            };
            Some(Number { n })
        } else {
            None
        }
    }

    /// If the `Number` is an integer, represent it as i128 if possible. Returns
    /// None otherwise.
    pub fn as_i128(&self) -> Option<i128> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as i128),
            N::NegInt(n) => Some(n as i128),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }

    /// If the `Number` is an integer, represent it as u128 if possible. Returns
    /// None otherwise.
    pub fn as_u128(&self) -> Option<u128> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as u128),
            N::NegInt(_) | N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }

    /// Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or
    /// larger than u64::MAX can only be represented in `Number` if serde_json's
    /// "arbitrary_precision" feature is enabled.
    ///
    /// ```
    /// # use serde_json::Number;
    /// #
    /// assert!(Number::from_i128(256).is_some());
    /// ```
    pub fn from_i128(i: i128) -> Option<Number> {
        let n = {
            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(u) = u64::try_from(i) {
                    N::PosInt(u)
                } else if let Ok(i) = i64::try_from(i) {
                    N::NegInt(i)
                } else {
                    return None;
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                i.to_string()
            }
        };
        Some(Number { n })
    }

    /// Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only
    /// be represented in `Number` if serde_json's "arbitrary_precision" feature
    /// is enabled.
    ///
    /// ```
    /// # use serde_json::Number;
    /// #
    /// assert!(Number::from_u128(256).is_some());
    /// ```
    pub fn from_u128(i: u128) -> Option<Number> {
        let n = {
            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(u) = u64::try_from(i) {
                    N::PosInt(u)
                } else {
                    return None;
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                i.to_string()
            }
        };
        Some(Number { n })
    }

    /// Returns the exact original JSON representation that this Number was
    /// parsed from.
    ///
    /// For numbers constructed not via parsing, such as by `From<i32>`, returns
    /// the JSON representation that serde\_json would serialize for this
    /// number.
    ///
    /// ```
    /// # use serde_json::Number;
    /// for value in [
    ///     "7",
    ///     "12.34",
    ///     "34e-56789",
    ///     "0.0123456789000000012345678900000001234567890000123456789",
    ///     "343412345678910111213141516171819202122232425262728293034",
    ///     "-343412345678910111213141516171819202122232425262728293031",
    /// ] {
    ///     let number: Number = serde_json::from_str(value).unwrap();
    ///     assert_eq!(number.as_str(), value);
    /// }
    /// ```
    #[cfg(feature = "arbitrary_precision")]
    #[cfg_attr(docsrs, doc(cfg(feature = "arbitrary_precision")))]
    pub fn as_str(&self) -> &str {
        &self.n
    }

    pub(crate) fn as_f32(&self) -> Option<f32> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f32),
            N::NegInt(n) => Some(n as f32),
            N::Float(n) => Some(n as f32),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f32>().ok().filter(|float| float.is_finite())
    }

    pub(crate) fn from_f32(f: f32) -> Option<Number> {
        if f.is_finite() {
            let n = {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::Float(f as f64)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    ryu::Buffer::new().format_finite(f).to_owned()
                }
            };
            Some(Number { n })
        } else {
            None
        }
    }

    #[cfg(feature = "arbitrary_precision")]
    /// Not public API. Only tests use this.
    #[doc(hidden)]
    #[inline]
    pub fn from_string_unchecked(n: String) -> Self {
        Number { n }
    }
}

impl Display for Number {
    #[cfg(not(feature = "arbitrary_precision"))]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
            N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
            N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }

    #[cfg(feature = "arbitrary_precision")]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.n, formatter)
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Number({})", self)
    }
}

impl Serialize for Number {
    #[cfg(not(feature = "arbitrary_precision"))]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(u) => serializer.serialize_u64(u),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }

    #[cfg(feature = "arbitrary_precision")]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = tri!(serializer.serialize_struct(TOKEN, 1));
        tri!(s.serialize_field(TOKEN, &self.n));
        s.end()
    }
}

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON number")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }

            fn visit_i128<E>(self, value: i128) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_i128(value)
                    .ok_or_else(|| de::Error::custom("JSON number out of range"))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }

            fn visit_u128<E>(self, value: u128) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_u128(value)
                    .ok_or_else(|| de::Error::custom("JSON number out of range"))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Number, E>
            where
                E: de::Error,
            {
                Number::from_f64(value).ok_or_else(|| de::Error::custom("not a JSON number"))
            }

            #[cfg(feature = "arbitrary_precision")]
            fn visit_map<V>(self, mut visitor: V) -> Result<Number, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let value = tri!(visitor.next_key::<NumberKey>());
                if value.is_none() {
                    return Err(de::Error::invalid_type(Unexpected::Map, &self));
                }
                let v: NumberFromString = tri!(visitor.next_value());
                Ok(v.value)
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

#[cfg(feature = "arbitrary_precision")]
struct NumberKey;

#[cfg(feature = "arbitrary_precision")]
impl<'de> de::Deserialize<'de> for NumberKey {
    fn deserialize<D>(deserializer: D) -> Result<NumberKey, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid number field")
            }

            fn visit_str<E>(self, s: &str) -> Result<(), E>
            where
                E: de::Error,
            {
                if s == TOKEN {
                    Ok(())
                } else {
                    Err(de::Error::custom("expected field with custom name"))
                }
            }
        }

        tri!(deserializer.deserialize_identifier(FieldVisitor));
        Ok(NumberKey)
    }
}

#[cfg(feature = "arbitrary_precision")]
pub struct NumberFromString {
    pub value: Number,
}

#[cfg(feature = "arbitrary_precision")]
impl<'de> de::Deserialize<'de> for NumberFromString {
    fn deserialize<D>(deserializer: D) -> Result<NumberFromString, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = NumberFromString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string containing a number")
            }

            fn visit_str<E>(self, s: &str) -> Result<NumberFromString, E>
            where
                E: de::Error,
            {
                let n = tri!(s.parse().map_err(de::Error::custom));
                Ok(NumberFromString { value: n })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "arbitrary_precision")]
fn invalid_number() -> Error {
    Error::syntax(ErrorCode::InvalidNumber, 0, 0)
}

macro_rules! deserialize_any {
    (@expand [$($num_string:tt)*]) => {
        #[cfg(not(feature = "arbitrary_precision"))]
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            match self.n {
                N::PosInt(u) => visitor.visit_u64(u),
                N::NegInt(i) => visitor.visit_i64(i),
                N::Float(f) => visitor.visit_f64(f),
            }
        }

        #[cfg(feature = "arbitrary_precision")]
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
            where V: Visitor<'de>
        {
            if let Some(u) = self.as_u64() {
                return visitor.visit_u64(u);
            } else if let Some(i) = self.as_i64() {
                return visitor.visit_i64(i);
            } else if let Some(u) = self.as_u128() {
                return visitor.visit_u128(u);
            } else if let Some(i) = self.as_i128() {
                return visitor.visit_i128(i);
            } else if let Some(f) = self.as_f64() {
                if ryu::Buffer::new().format_finite(f) == self.n || f.to_string() == self.n {
                    return visitor.visit_f64(f);
                }
            }

            visitor.visit_map(NumberDeserializer {
                number: Some(self.$($num_string)*),
            })
        }
    };

    (owned) => {
        deserialize_any!(@expand [n]);
    };

    (ref) => {
        deserialize_any!(@expand [n.clone()]);
    };
}

macro_rules! deserialize_number {
    ($deserialize:ident => $visit:ident) => {
        #[cfg(not(feature = "arbitrary_precision"))]
        fn $deserialize<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_any(visitor)
        }

        #[cfg(feature = "arbitrary_precision")]
        fn $deserialize<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: de::Visitor<'de>,
        {
            visitor.$visit(tri!(self.n.parse().map_err(|_| invalid_number())))
        }
    };
}

impl<'de> Deserializer<'de> for Number {
    type Error = Error;

    deserialize_any!(owned);

    deserialize_number!(deserialize_i8 => visit_i8);
    deserialize_number!(deserialize_i16 => visit_i16);
    deserialize_number!(deserialize_i32 => visit_i32);
    deserialize_number!(deserialize_i64 => visit_i64);
    deserialize_number!(deserialize_i128 => visit_i128);
    deserialize_number!(deserialize_u8 => visit_u8);
    deserialize_number!(deserialize_u16 => visit_u16);
    deserialize_number!(deserialize_u32 => visit_u32);
    deserialize_number!(deserialize_u64 => visit_u64);
    deserialize_number!(deserialize_u128 => visit_u128);
    deserialize_number!(deserialize_f32 => visit_f32);
    deserialize_number!(deserialize_f64 => visit_f64);

    forward_to_deserialize_any! {
        bool char str string bytes byte_buf option unit unit_struct
        newtype_struct seq tuple tuple_struct map struct enum identifier
        ignored_any
    }
}

impl<'de> Deserializer<'de> for &Number {
    type Error = Error;

    deserialize_any!(ref);

    deserialize_number!(deserialize_i8 => visit_i8);
    deserialize_number!(deserialize_i16 => visit_i16);
    deserialize_number!(deserialize_i32 => visit_i32);
    deserialize_number!(deserialize_i64 => visit_i64);
    deserialize_number!(deserialize_i128 => visit_i128);
    deserialize_number!(deserialize_u8 => visit_u8);
    deserialize_number!(deserialize_u16 => visit_u16);
    deserialize_number!(deserialize_u32 => visit_u32);
    deserialize_number!(deserialize_u64 => visit_u64);
    deserialize_number!(deserialize_u128 => visit_u128);
    deserialize_number!(deserialize_f32 => visit_f32);
    deserialize_number!(deserialize_f64 => visit_f64);

    forward_to_deserialize_any! {
        bool char str string bytes byte_buf option unit unit_struct
        newtype_struct seq tuple tuple_struct map struct enum identifier
        ignored_any
    }
}

#[cfg(feature = "arbitrary_precision")]
pub(crate) struct NumberDeserializer {
    pub number: Option<String>,
}

#[cfg(feature = "arbitrary_precision")]
impl<'de> MapAccess<'de> for NumberDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.number.is_none() {
            return Ok(None);
        }
        seed.deserialize(NumberFieldDeserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.number.take().unwrap().into_deserializer())
    }
}

#[cfg(feature = "arbitrary_precision")]
struct NumberFieldDeserializer;

#[cfg(feature = "arbitrary_precision")]
impl<'de> Deserializer<'de> for NumberFieldDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(TOKEN)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string seq
        bytes byte_buf map struct option unit newtype_struct ignored_any
        unit_struct tuple_struct tuple enum identifier
    }
}

impl From<ParserNumber> for Number {
    fn from(value: ParserNumber) -> Self {
        let n = match value {
            ParserNumber::F64(f) => {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::Float(f)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    ryu::Buffer::new().format_finite(f).to_owned()
                }
            }
            ParserNumber::U64(u) => {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::PosInt(u)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    itoa::Buffer::new().format(u).to_owned()
                }
            }
            ParserNumber::I64(i) => {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::NegInt(i)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    itoa::Buffer::new().format(i).to_owned()
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(s) => s,
        };
        Number { n }
    }
}

macro_rules! impl_from_unsigned {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Number {
                fn from(u: $ty) -> Self {
                    let n = {
                        #[cfg(not(feature = "arbitrary_precision"))]
                        { N::PosInt(u as u64) }
                        #[cfg(feature = "arbitrary_precision")]
                        {
                            itoa::Buffer::new().format(u).to_owned()
                        }
                    };
                    Number { n }
                }
            }
        )*
    };
}

macro_rules! impl_from_signed {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Number {
                fn from(i: $ty) -> Self {
                    let n = {
                        #[cfg(not(feature = "arbitrary_precision"))]
                        {
                            if i < 0 {
                                N::NegInt(i as i64)
                            } else {
                                N::PosInt(i as u64)
                            }
                        }
                        #[cfg(feature = "arbitrary_precision")]
                        {
                            itoa::Buffer::new().format(i).to_owned()
                        }
                    };
                    Number { n }
                }
            }
        )*
    };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);

#[cfg(feature = "arbitrary_precision")]
impl_from_unsigned!(u128);
#[cfg(feature = "arbitrary_precision")]
impl_from_signed!(i128);

impl Number {
    #[cfg(not(feature = "arbitrary_precision"))]
    #[cold]
    pub(crate) fn unexpected(&self) -> Unexpected {
        match self.n {
            N::PosInt(u) => Unexpected::Unsigned(u),
            N::NegInt(i) => Unexpected::Signed(i),
            N::Float(f) => Unexpected::Float(f),
        }
    }

    #[cfg(feature = "arbitrary_precision")]
    #[cold]
    pub(crate) fn unexpected(&self) -> Unexpected {
        Unexpected::Other("number")
    }
}

#[cfg(test)]
mod tests_llm_16_88 {
    use crate::number::Number;
    use crate::de::Deserializer;
    use serde::Deserialize;

    #[test]
    fn test_deserialize_i128() {
        let json_value = r#""-9223372036854775809""#; // Example JSON representing an i128
        let de_value: Result<Number, _> = crate::from_str(json_value);
        assert!(de_value.is_err()); // Out of range for i128

        let json_value = r#""9223372036854775807""#; // Max i128
        let de_value: Result<Number, _> = crate::from_str(json_value);
        assert!(de_value.is_ok()); // Should deserialize correctly

        let number = de_value.unwrap();
        assert_eq!(number.as_i128(), Some(9223372036854775807));
    }
}

#[cfg(test)]
mod tests_llm_16_104 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_deserialize_u128() {
        let input = crate::json!(12345678901234567890_u128);
        let number: Number = crate::from_value(input).unwrap();
        assert_eq!(number.as_u128(), Some(12345678901234567890_u128));

        let input = crate::json!(18446744073709551615_u128);
        let number: Number = crate::from_value(input).unwrap();
        assert_eq!(number.as_u128(), Some(18446744073709551615_u128));

        let input = crate::json!(-1);
        let number: Number = crate::from_value(input).unwrap();
        assert_eq!(number.as_u128(), None);

        let input = crate::json!(3.14);
        let number: Number = crate::from_value(input).unwrap();
        assert_eq!(number.as_u128(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_228 {
    use super::*;

use crate::*;
    use crate::number::Number;
    use crate::de::ParserNumber;

    #[test]
    fn test_from_f64() {
        let parser_number = ParserNumber::F64(3.14);
        let number: Number = Number::from(parser_number);
        assert!(number.is_f64());
        assert_eq!(number.as_f64(), Some(3.14));
    }

    #[test]
    fn test_from_u64() {
        let parser_number = ParserNumber::U64(42);
        let number: Number = Number::from(parser_number);
        assert!(number.is_u64());
        assert_eq!(number.as_u64(), Some(42));
    }

    #[test]
    fn test_from_i64() {
        let parser_number = ParserNumber::I64(-10);
        let number: Number = Number::from(parser_number);
        assert!(number.is_i64());
        assert_eq!(number.as_i64(), Some(-10));
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_from_string() {
        let parser_number = ParserNumber::String("12345678901234567890".to_string());
        let number: Number = Number::from(parser_number);
        assert!(number.is_f64());
        assert_eq!(number.as_str(), "12345678901234567890");
    }
}

#[cfg(test)]
mod tests_llm_16_229 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_positive_i16() {
        let number: Number = Number::from(42_i16);
        assert!(number.is_u64());
        assert_eq!(number.as_u64(), Some(42_u64));
    }

    #[test]
    fn test_from_negative_i16() {
        let number: Number = Number::from(-42_i16);
        assert!(number.is_i64());
        assert_eq!(number.as_i64(), Some(-42_i64));
    }

    #[test]
    fn test_from_zero_i16() {
        let number: Number = Number::from(0_i16);
        assert!(number.is_u64());
        assert_eq!(number.as_u64(), Some(0_u64));
    }
}

#[cfg(test)]
mod tests_llm_16_230 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_i32_positive() {
        let num: Number = Number::from(42);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(42));
        assert_eq!(num.as_i64(), Some(42));
        assert_eq!(num.as_f64(), Some(42.0));
    }

    #[test]
    fn test_from_i32_negative() {
        let num: Number = Number::from(-42);
        assert!(num.is_i64());
        assert_eq!(num.as_u64(), None);
        assert_eq!(num.as_i64(), Some(-42));
        assert_eq!(num.as_f64(), Some(-42.0));
    }

    #[test]
    fn test_from_i32_zero() {
        let num: Number = Number::from(0);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(0));
        assert_eq!(num.as_i64(), Some(0));
        assert_eq!(num.as_f64(), Some(0.0));
    }
}

#[cfg(test)]
mod tests_llm_16_232 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_from_i8_positive() {
        let number: Number = Number::from(5_i8);
        assert!(number.is_u64());
        assert_eq!(number.as_u64(), Some(5_u64));
    }

    #[test]
    fn test_from_i8_negative() {
        let number: Number = Number::from(-5_i8);
        assert!(number.is_i64());
        assert_eq!(number.as_i64(), Some(-5_i64));
    }

    #[test]
    fn test_from_i8_zero() {
        let number: Number = Number::from(0_i8);
        assert!(number.is_u64());
        assert_eq!(number.as_u64(), Some(0_u64));
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_from_i8_arbitrary_precision() {
        let number: Number = Number::from(100_i8);
        assert_eq!(number.as_str(), "100");
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_from_i8_arbitrary_precision_negative() {
        let number: Number = Number::from(-100_i8);
        assert_eq!(number.as_str(), "-100");
    }
}

#[cfg(test)]
mod tests_llm_16_233 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_isize_positive() {
        let num: Number = Number::from(42isize);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(42));
    }

    #[test]
    fn test_from_isize_negative() {
        let num: Number = Number::from(-42isize);
        assert!(num.is_i64());
        assert_eq!(num.as_i64(), Some(-42));
    }

    #[test]
    fn test_from_isize_zero() {
        let num: Number = Number::from(0isize);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(0));
    }
    
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_from_isize_arbitrary_precision() {
        let num: Number = Number::from(12345678901234567890isize);
        assert!(num.is_f64());
    }
    
    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_from_isize_arbitrary_precision_negative() {
        let num: Number = Number::from(-12345678901234567890isize);
        assert!(num.is_f64());
    }
}

#[cfg(test)]
mod tests_llm_16_234 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_u16() {
        let num: Number = Number::from(42u16);
        assert_eq!(num.as_u64(), Some(42));
        assert!(num.is_u64());
        assert!(!num.is_i64());
        assert!(!num.is_f64());
    }

    #[test]
    fn test_from_u16_zero() {
        let num: Number = Number::from(0u16);
        assert_eq!(num.as_u64(), Some(0));
        assert!(num.is_u64());
        assert!(!num.is_i64());
        assert!(!num.is_f64());
    }

    #[test]
    fn test_from_u16_large() {
        let num: Number = Number::from(65535u16);
        assert_eq!(num.as_u64(), Some(65535));
        assert!(num.is_u64());
        assert!(!num.is_i64());
        assert!(!num.is_f64());
    }
}

#[cfg(test)]
mod tests_llm_16_235 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_u32() {
        let num: Number = Number::from(42u32);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(42));
        assert_eq!(num.as_i64(), Some(42));
        assert_eq!(num.as_f64(), Some(42.0));
    }

    #[test]
    fn test_from_u32_zero() {
        let num: Number = Number::from(0u32);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(0));
        assert_eq!(num.as_i64(), Some(0));
        assert_eq!(num.as_f64(), Some(0.0));
    }

    #[test]
    fn test_from_u32_max() {
        let num: Number = Number::from(u32::MAX);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(u32::MAX as u64));
        assert_eq!(num.as_i64(), Some(u32::MAX as i64));
        assert!(num.as_f64() == Some(u32::MAX as f64));
    }
}

#[cfg(test)]
mod tests_llm_16_236 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_u64() {
        let num = Number::from(42u64);
        assert_eq!(num.as_u64(), Some(42));
        assert!(num.is_u64());
        assert!(!num.is_i64());
        assert!(!num.is_f64());

        let num = Number::from(0u64);
        assert_eq!(num.as_u64(), Some(0));
        assert!(num.is_u64());
        
        let num = Number::from(u64::MAX);
        assert_eq!(num.as_u64(), Some(u64::MAX));
        assert!(num.is_u64());
    }

    #[test]
    #[cfg(not(feature = "arbitrary_precision"))]
    fn test_from_negative_u64() {
        let num = Number::from(0u64);
        assert_eq!(num.as_i64(), Some(0));
        assert!(num.is_i64());
    }

    #[test]
    #[cfg(feature = "arbitrary_precision")]
    fn test_from_u64_arbitrary_precision() {
        // Test from u64 when arbitrary precision is enabled
        let num = Number::from(12345678901234567890u64);
        assert!(num.as_str().parse::<u64>().is_ok());
    }
}

#[cfg(test)]
mod tests_llm_16_237 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_u8_positive() {
        let num: Number = Number::from(10u8);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(10));
    }

    #[test]
    fn test_from_u8_zero() {
        let num: Number = Number::from(0u8);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(0));
    }

    #[test]
    fn test_from_u8_max() {
        let num: Number = Number::from(255u8);
        assert!(num.is_u64());
        assert_eq!(num.as_u64(), Some(255));
    }

    #[test]
    fn test_from_u8_arbitrary_precision() {
        #[cfg(feature = "arbitrary_precision")]
        {
            let num: Number = Number::from(255u8);
            assert!(num.as_str().parse::<u8>().is_ok());
        }
    }
}

#[cfg(test)]
mod tests_llm_16_250 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_deserialize_i128() {
        // Test positive i128
        let num: Number = crate::from_str("123456789123456789123456789").unwrap();
        assert_eq!(num.as_i128(), Some(123456789123456789123456789));

        // Test negative i128
        let num: Number = crate::from_str("-123456789123456789123456789").unwrap();
        assert_eq!(num.as_i128(), Some(-123456789123456789123456789));

        // Test out of range i128
        let num: Number = crate::from_str("123456789123456789123456789123456789").unwrap();
        assert_eq!(num.as_i128(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_253 {
    use crate::Number;
    use crate::de;

    #[test]
    fn test_deserialize_i64_pos_int() {
        let num: Number = crate::from_str("42").unwrap();
        assert_eq!(num.as_i64(), Some(42));
    }

    #[test]
    fn test_deserialize_i64_neg_int() {
        let num: Number = crate::from_str("-42").unwrap();
        assert_eq!(num.as_i64(), Some(-42));
    }

    #[test]
    fn test_deserialize_i64_float() {
        let num: Number = crate::from_str("42.0").unwrap();
        assert_eq!(num.as_i64(), None);
    }

    #[test]
    fn test_deserialize_i64_large_pos_int() {
        let num: Number = crate::from_str("9223372036854775807").unwrap();
        assert_eq!(num.as_i64(), None);
    }

    #[test]
    fn test_deserialize_i64_large_neg_int() {
        let num: Number = crate::from_str("-9223372036854775808").unwrap();
        assert_eq!(num.as_i64(), Some(-9223372036854775808));
    }

    #[test]
    fn test_deserialize_i64_invalid() {
        let result: Result<Number, _> = crate::from_str("\"not a number\"");
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_llm_16_254 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_deserialize_i8() {
        // Test case for positive i8 value
        let json_value = crate::json!(42);
        let number: Number = crate::from_value(json_value).unwrap();
        assert_eq!(number.as_i64(), Some(42));

        // Test case for negative i8 value
        let json_value = crate::json!(-42);
        let number: Number = crate::from_value(json_value).unwrap();
        assert_eq!(number.as_i64(), Some(-42));

        // Test case for value that cannot be deserialized to i8
        let json_value = crate::json!(256);
        let number: Number = crate::from_value(json_value).unwrap();
        assert!(number.as_i64().is_none());

        // Test case for float value
        let json_value = crate::json!(42.5);
        let number: Number = crate::from_value(json_value).unwrap();
        assert!(number.as_i64().is_none());

        // Test case for zero
        let json_value = crate::json!(0);
        let number: Number = crate::from_value(json_value).unwrap();
        assert_eq!(number.as_i64(), Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_664 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_as_f32_pos_int() {
        let num = Number::from(42u64);
        assert_eq!(num.as_f32(), Some(42.0));
    }

    #[test]
    fn test_as_f32_neg_int() {
        let num = Number::from(-42i64);
        assert_eq!(num.as_f32(), Some(-42.0));
    }

    #[test]
    fn test_as_f32_float() {
        let num = Number::from_f64(3.14).unwrap();
        assert_eq!(num.as_f32(), Some(3.14_f32));
    }

    #[test]
    fn test_as_f32_infinite() {
        let num = Number::from_f64(f64::INFINITY).unwrap();
        assert_eq!(num.as_f32(), None);
    }

    #[test]
    fn test_as_f32_nan() {
        let num = Number::from_f64(f64::NAN).unwrap();
        assert_eq!(num.as_f32(), None);
    }

    #[test]
    fn test_as_f32_feature_arbitrary_precision() {
        #[cfg(feature = "arbitrary_precision")]
        {
            let num: Number = Number::from_string_unchecked("3.14".to_string());
            assert_eq!(num.as_f32(), Some(3.14_f32));
        }
    }
}

#[cfg(test)]
mod tests_llm_16_667 {
    use crate::Number; // adjust the import path according to your project structure
    use crate::number::N; // adjust the import path according to your project structure

    #[test]
    fn test_as_i64_pos_int() {
        let num = Number { n: N::PosInt(42) };
        assert_eq!(num.as_i64(), Some(42));
    }

    #[test]
    fn test_as_i64_neg_int() {
        let num = Number { n: N::NegInt(-42) };
        assert_eq!(num.as_i64(), Some(-42));
    }

    #[test]
    fn test_as_i64_float() {
        let num = Number { n: N::Float(42.0) };
        assert_eq!(num.as_i64(), None);
    }

    #[test]
    fn test_as_i64_large_pos_int() {
        let num = Number { n: N::PosInt(u64::MAX) };
        assert_eq!(num.as_i64(), None);
    }

    #[test]
    fn test_as_i64_large_neg_int() {
        let num = Number { n: N::PosInt(i64::MAX as u64 + 1) };
        assert_eq!(num.as_i64(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_668 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_as_u128_positive_integer() {
        let num = Number { n: N::PosInt(100) };
        assert_eq!(num.as_u128(), Some(100));
    }

    #[test]
    fn test_as_u128_negative_integer() {
        let num = Number { n: N::NegInt(-100) };
        assert_eq!(num.as_u128(), None);
    }

    #[test]
    fn test_as_u128_float() {
        let num = Number { n: N::Float(100.0) };
        assert_eq!(num.as_u128(), None);
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_as_u128_arbitrary_precision() {
        let num = Number::from_u128(100u128).unwrap();
        assert_eq!(num.as_u128(), Some(100));
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_as_u128_arbitrary_precision_large() {
        let num = Number::from_u128(u128::MAX).unwrap();
        assert_eq!(num.as_u128(), Some(u128::MAX));
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_as_u128_arbitrary_precision_out_of_bounds() {
        let num = Number::from_i128(i128::MAX).unwrap();
        assert_eq!(num.as_u128(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_669 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_as_u64_pos_int() {
        let num = Number { n: N::PosInt(42) };
        assert_eq!(num.as_u64(), Some(42));
    }

    #[test]
    fn test_as_u64_neg_int() {
        let num = Number { n: N::NegInt(-42) };
        assert_eq!(num.as_u64(), None);
    }

    #[test]
    fn test_as_u64_float() {
        let num = Number { n: N::Float(42.0) };
        assert_eq!(num.as_u64(), None);
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_as_u64_arbitrary_precision() {
        let num = Number::from_string_unchecked("100".to_string());
        assert_eq!(num.as_u64(), Some(100));
        
        let num = Number::from_string_unchecked("-100".to_string());
        assert_eq!(num.as_u64(), None);
        
        let num = Number::from_string_unchecked("12.34".to_string());
        assert_eq!(num.as_u64(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_671 {
    use super::*; // Assuming `from_f64` is in the same module

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_f64_finite() {
        assert!(Number::from_f64(256.0).is_some());
        assert!(Number::from_f64(0.0).is_some());
        assert!(Number::from_f64(-123.456).is_some());
        assert!(Number::from_f64(1.0e10).is_some());
    }

    #[test]
    fn test_from_f64_infinite_nan() {
        assert!(Number::from_f64(f64::INFINITY).is_none());
        assert!(Number::from_f64(f64::NEG_INFINITY).is_none());
        assert!(Number::from_f64(f64::NAN).is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_672 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_i128() {
        // Test within valid range
        assert_eq!(Number::from_i128(256), Some(Number::from(256)));
        assert_eq!(Number::from_i128(-256), Some(Number::from(-256)));
        assert_eq!(Number::from_i128(0), Some(Number::from(0)));

        // Test out of range for i64
        assert_eq!(Number::from_i128(i128::MIN), None);
        assert_eq!(Number::from_i128(i128::MAX), None);

        // Test precision feature
        #[cfg(feature = "arbitrary_precision")]
        {
            assert_eq!(Number::from_i128(i128::MAX), Some(Number::from(i128::MAX.to_string())));
            assert_eq!(Number::from_i128(i128::MIN), Some(Number::from(i128::MIN.to_string())));
        }
    }
}

#[cfg(test)]
mod tests_llm_16_673 {
    use super::*;

use crate::*;
    use crate::Number;

    #[test]
    fn test_from_u128_within_u64_range() {
        let result = Number::from_u128(256);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Number::from(256u64));
    }

    #[test]
    fn test_from_u128_exceeding_u64_max() {
        #[cfg(not(feature = "arbitrary_precision"))]
        {
            let result = Number::from_u128(u128::from(u64::MAX) + 1);
            assert!(result.is_none());
        }

        #[cfg(feature = "arbitrary_precision")]
        {
            let result = Number::from_u128(u128::from(u64::MAX) + 1);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), Number::from((u64::MAX as u128) + 1));
        }
    }

    #[test]
    fn test_from_u128_zero() {
        let result = Number::from_u128(0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Number::from(0u64));
    }

    #[test]
    fn test_from_u128_max_value() {
        let result = Number::from_u128(u128::MAX);
        #[cfg(not(feature = "arbitrary_precision"))]
        {
            assert!(result.is_none());
        }

        #[cfg(feature = "arbitrary_precision")]
        {
            assert!(result.is_some());
            assert_eq!(result.unwrap(), Number::from(u128::MAX.to_string()));
        }
    }
}

#[cfg(test)]
mod tests_llm_16_674 {
    use super::*; // Import everything from the parent module

use crate::*;
    use crate::Number;

    #[test]
    fn test_is_f64() {
        // Test cases for `is_f64`
        let float_number = Number { n: N::Float(3.14) };
        let pos_int_number = Number { n: N::PosInt(42) };
        let neg_int_number = Number { n: N::NegInt(-42) };

        assert!(float_number.is_f64());
        assert!(!pos_int_number.is_f64());
        assert!(!neg_int_number.is_f64());
    }

    #[cfg(feature = "arbitrary_precision")]
    #[test]
    fn test_is_f64_arbitrary_precision() {
        // Test cases for `is_f64` with arbitrary precision
        let float_number = Number::from_string_unchecked("3.14".to_string());
        let scientific_number = Number::from_string_unchecked("1e10".to_string());
        let integer_string_number = Number::from_string_unchecked("42".to_string());

        assert!(float_number.is_f64());
        assert!(scientific_number.is_f64());
        assert!(!integer_string_number.is_f64());
    }
}

#[cfg(test)]
mod tests_llm_16_675 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_is_i64_positive_within_limit() {
        let number = Number { n: N::PosInt(42) };
        assert!(number.is_i64());
    }

    #[test]
    fn test_is_i64_negative_within_limit() {
        let number = Number { n: N::NegInt(-42) };
        assert!(number.is_i64());
    }

    #[test]
    fn test_is_i64_positive_out_of_limit() {
        let number = Number { n: N::PosInt(i64::MAX as u64 + 1) };
        assert!(!number.is_i64());
    }

    #[test]
    fn test_is_i64_float() {
        let number = Number { n: N::Float(3.14) };
        assert!(!number.is_i64());
    }
}
