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
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::str::FromStr;
	use std::iter::Iterator;
	use read::Read;
	use std::iter::DoubleEndedIterator;
	use std::cmp::Eq;
	use std::ops::Deref;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut usize_0: usize = 3217usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_0);
    let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
    let mut str_0: &str = "0GDOW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &crate::read::StrRead = &mut strread_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut values_0: crate::map::Values = crate::map::Map::values(map_1_ref_0);
    let mut values_0_ref_0: &mut crate::map::Values = &mut values_0;
    let mut i64_0: i64 = 817i64;
    let mut n_0: number::N = crate::number::N::NegInt(i64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut f64_0: f64 = -13407.790916f64;
    let mut f64_1: f64 = 10496.141587f64;
    let mut n_1: number::N = crate::number::N::Float(f64_1);
    let mut bool_0: bool = true;
    let mut value_0: value::Value = crate::value::Value::Bool(bool_0);
    let mut value_1: value::Value = crate::value::Value::Null;
    let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
    let mut category_0: error::Category = crate::error::Category::Io;
    let mut option_0: std::option::Option<crate::number::Number> = crate::number::Number::from_f64(f64_0);
    let mut option_1: std::option::Option<i128> = crate::number::Number::as_i128(number_0_ref_0);
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut bool_1: bool = crate::number::Number::is_u64(number_1_ref_0);
    let mut prettyformatter_0: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::new();
    let mut option_2: std::option::Option<&value::Value> = crate::map::Values::next_back(values_0_ref_0);
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut position_0: crate::read::Position = crate::read::StrRead::peek_position(strread_0_ref_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::FormFeed;
    let mut option_3: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_10() {
//     rusty_monitor::set_test_id(10);
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut usize_0: usize = 6508usize;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut u64_0: u64 = 6245u64;
//     let mut n_0: number::N = crate::number::N::PosInt(u64_0);
//     let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
//     let mut str_0: &str = "2dGGdww3X";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut usize_1: usize = 8789usize;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
//     let mut map_2_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_3_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_3;
//     let mut valuesmut_0: crate::map::ValuesMut = crate::map::Map::values_mut(map_3_ref_0);
//     let mut valuesmut_0_ref_0: &crate::map::ValuesMut = &mut valuesmut_0;
//     let mut str_1: &str = "paio";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut i64_0: i64 = -6154i64;
//     let mut n_1: number::N = crate::number::N::NegInt(i64_0);
//     let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
//     let mut number_1_ref_0: &crate::number::Number = &mut number_1;
//     let mut option_0: std::option::Option<i64> = crate::number::Number::as_i64(number_1_ref_0);
//     crate::de::from_str(str_1_ref_0);
//     let mut usize_2: usize = crate::map::ValuesMut::len(valuesmut_0_ref_0);
//     let mut i64_1: i64 = std::option::Option::unwrap(option_0);
//     crate::map::Map::sort_keys(map_2_ref_0);
//     let mut deserializer_0: crate::de::Deserializer<crate::read::StrRead> = crate::de::Deserializer::from_str(str_0_ref_0);
//     let mut value_0: value::Value = crate::value::Value::Number(number_0);
//     crate::map::Map::clone_from(map_1_ref_0, map_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut valuesmut_0: crate::map::ValuesMut = crate::map::Map::values_mut(map_0_ref_0);
    let mut valuesmut_0_ref_0: &mut crate::map::ValuesMut = &mut valuesmut_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_2_ref_0);
    let mut keys_0_ref_0: &crate::map::Keys = &mut keys_0;
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_3_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_3;
    let mut u64_0: u64 = 6214u64;
    let mut n_0: number::N = crate::number::N::PosInt(u64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut f64_0: f64 = -7709.350675f64;
    let mut n_1: number::N = crate::number::N::Float(f64_0);
    let mut n_1_ref_0: &number::N = &mut n_1;
    let mut str_0: &str = "r9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut i64_0: i64 = -15918i64;
    let mut n_2: number::N = crate::number::N::NegInt(i64_0);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_2};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut bool_0: bool = crate::number::Number::is_f64(number_1_ref_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::CarriageReturn;
    let mut result_0: std::result::Result<crate::number::Number, crate::error::Error> = crate::number::Number::from_str(str_0_ref_0);
    let mut number_2: crate::number::Number = std::result::Result::unwrap(result_0);
    let mut n_3: number::N = crate::number::N::clone(n_1_ref_0);
    let mut usize_0: usize = crate::map::Map::len(map_3_ref_0);
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Keys::size_hint(keys_0_ref_0);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_1);
    let mut option_0: std::option::Option<&mut value::Value> = crate::map::ValuesMut::next(valuesmut_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut i64_0: i64 = 17588i64;
    let mut n_0: number::N = crate::number::N::NegInt(i64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut usize_0: usize = 9140usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut i64_1: i64 = 6515i64;
    let mut n_1: number::N = crate::number::N::NegInt(i64_1);
    let mut n_1_ref_0: &number::N = &mut n_1;
    let mut u64_0: u64 = 4021u64;
    let mut n_2: number::N = crate::number::N::PosInt(u64_0);
    let mut n_2_ref_0: &number::N = &mut n_2;
    let mut u64_1: u64 = 6112u64;
    let mut n_3: number::N = crate::number::N::PosInt(u64_1);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_3};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut u64_2: u64 = 9628u64;
    let mut n_4: number::N = crate::number::N::PosInt(u64_2);
    let mut number_2: crate::number::Number = crate::number::Number {n: n_4};
    let mut number_2_ref_0: &crate::number::Number = &mut number_2;
    let mut f32_0: f32 = -3626.718547f32;
    let mut option_0: std::option::Option<crate::number::Number> = crate::number::Number::from_f32(f32_0);
    let mut option_1: std::option::Option<f64> = crate::number::Number::as_f64(number_1_ref_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::ReverseSolidus;
    let mut bool_0: bool = crate::number::N::eq(n_2_ref_0, n_1_ref_0);
    let mut number_3: crate::number::Number = std::option::Option::unwrap(option_0);
    let mut itermut_0: crate::map::IterMut = crate::map::Map::iter_mut(map_0_ref_0);
    let mut bool_1: bool = crate::number::Number::is_i64(number_0_ref_0);
    let mut charescape_1: ser::CharEscape = crate::ser::CharEscape::Quote;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut i64_0: i64 = -1659i64;
    let mut state_0: ser::State = crate::ser::State::Rest;
    let mut state_0_ref_0: &ser::State = &mut state_0;
    let mut vec_0: std::vec::Vec<value::Value> = std::vec::Vec::new();
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut itermut_0: crate::map::IterMut = crate::map::Map::iter_mut(map_0_ref_0);
    let mut itermut_0_ref_0: &mut crate::map::IterMut = &mut itermut_0;
    let mut u8_0: u8 = 107u8;
    let mut isize_0: isize = -5705isize;
    let mut usize_0: usize = 2678usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut itermut_1: crate::map::IterMut = crate::map::Map::iter_mut(map_1_ref_0);
    let mut itermut_1_ref_0: &crate::map::IterMut = &mut itermut_1;
    let mut str_0: &str = "SCyfoCzG5NTz7Zi5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_1: usize = 7919usize;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut values_0: crate::map::Values = crate::map::Map::values(map_2_ref_0);
    let mut values_0_ref_0: &crate::map::Values = &mut values_0;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Values::size_hint(values_0_ref_0);
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::IterMut::size_hint(itermut_1_ref_0);
    let mut strread_0_ref_0: &crate::read::StrRead = &mut strread_0;
    let mut position_0: crate::read::Position = crate::read::StrRead::position(strread_0_ref_0);
    let mut option_0: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next_back(itermut_0_ref_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::Quote;
    let mut tuple_2: () = crate::ser::State::assert_receiver_is_total_eq(state_0_ref_0);
    let mut n_0: number::N = crate::number::N::NegInt(i64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut value_0: value::Value = crate::value::Value::Null;
    let mut usize_0: usize = 600usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut str_0: &str = "Gtk5pO8p";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut isize_0: isize = 7411isize;
    let mut isize_0_ref_0: &isize = &mut isize_0;
    let mut reference_0: read::Reference<isize> = crate::read::Reference::Copied(isize_0_ref_0);
    let mut reference_0_ref_0: &read::Reference<isize> = &mut reference_0;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut f64_0: f64 = 6809.379729f64;
    let mut n_0: number::N = crate::number::N::Float(f64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut usize_1: usize = 5639usize;
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut map_3_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_3;
    crate::map::Map::sort_keys(map_3_ref_0);
    let mut bool_0: bool = crate::number::Number::is_i64(number_0_ref_0);
    let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_2_ref_0);
    let mut isize_1: &isize = crate::read::Reference::deref(reference_0_ref_0);
    let mut usize_2: usize = crate::map::Map::len(map_1_ref_0);
    crate::read::StrRead::peek(strread_0_ref_0);
    let mut iter_0_ref_0: &crate::map::Iter = &mut iter_0;
    let mut usize_3: usize = crate::map::Iter::len(iter_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_18() {
//     rusty_monitor::set_test_id(18);
//     let mut usize_0: usize = 3171usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_0);
//     let mut intovalues_0_ref_0: &mut crate::map::IntoValues = &mut intovalues_0;
//     let mut prettyformatter_0: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::default();
//     let mut prettyformatter_0_ref_0: &crate::ser::PrettyFormatter = &mut prettyformatter_0;
//     let mut isize_0: isize = -13393isize;
//     let mut isize_0_ref_0: &isize = &mut isize_0;
//     let mut reference_0: read::Reference<isize> = crate::read::Reference::Borrowed(isize_0_ref_0);
//     let mut reference_0_ref_0: &read::Reference<isize> = &mut reference_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::into_iter(map_1_ref_0);
//     let mut itermut_0_ref_0: &mut crate::map::IterMut = &mut itermut_0;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_2_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut u64_0: u64 = 1965u64;
//     let mut n_0: number::N = crate::number::N::PosInt(u64_0);
//     let mut n_0_ref_0: &number::N = &mut n_0;
//     let mut i64_0: i64 = -3008i64;
//     let mut n_1: number::N = crate::number::N::NegInt(i64_0);
//     let mut n_1_ref_0: &number::N = &mut n_1;
//     let mut bool_0: bool = crate::number::N::eq(n_1_ref_0, n_0_ref_0);
//     let mut state_0: ser::State = crate::ser::State::Empty;
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::Solidus;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut tuple_0: () = crate::ser::State::assert_receiver_is_total_eq(state_0_ref_0);
//     let mut itermut_1: crate::map::IterMut = crate::map::Map::into_iter(map_2_ref_0);
//     let mut option_0: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next(itermut_0_ref_0);
//     let mut itermut_1_ref_0: &mut crate::map::IterMut = &mut itermut_1;
//     let mut option_1: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next_back(itermut_1_ref_0);
//     let mut isize_1: &isize = crate::read::Reference::deref(reference_0_ref_0);
//     let mut prettyformatter_1: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::clone(prettyformatter_0_ref_0);
//     let mut option_2: std::option::Option<value::Value> = crate::map::IntoValues::next_back(intovalues_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_21() {
//     rusty_monitor::set_test_id(21);
//     let mut u64_0: u64 = 8989u64;
//     let mut n_0: number::N = crate::number::N::PosInt(u64_0);
//     let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
//     let mut number_0_ref_0: &crate::number::Number = &mut number_0;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_2_ref_0);
//     let mut iter_0_ref_0: &mut crate::map::Iter = &mut iter_0;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_3);
//     let mut intovalues_0_ref_0: &mut crate::map::IntoValues = &mut intovalues_0;
//     let mut i64_0: i64 = -14880i64;
//     let mut n_1: number::N = crate::number::N::NegInt(i64_0);
//     let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
//     let mut number_1_ref_0: &crate::number::Number = &mut number_1;
//     let mut usize_0: usize = 5107usize;
//     let mut option_0: std::option::Option<f32> = crate::number::Number::as_f32(number_1_ref_0);
//     let mut option_1: std::option::Option<value::Value> = crate::map::IntoValues::next(intovalues_0_ref_0);
//     let mut f32_0: f32 = std::option::Option::unwrap(option_0);
//     let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_4_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_4;
//     let mut iter_1: crate::map::Iter = crate::map::Map::iter(map_4_ref_0);
//     let mut option_2: std::option::Option<(&std::string::String, &value::Value)> = crate::map::Iter::next(iter_0_ref_0);
//     let mut tuple_0: (&std::string::String, &value::Value) = std::option::Option::unwrap(option_2);
//     let mut iter_2: crate::map::Iter = crate::map::Map::into_iter(map_1_ref_0);
//     let mut values_0: crate::map::Values = crate::map::Map::values(map_0_ref_0);
//     let mut iter_1_ref_0: &crate::map::Iter = &mut iter_1;
//     let mut usize_1: usize = crate::map::Iter::len(iter_1_ref_0);
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::Solidus;
//     let mut bool_0: bool = crate::number::Number::is_i64(number_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_24() {
//     rusty_monitor::set_test_id(24);
//     let mut usize_0: usize = 4451usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_0);
//     let mut intoiter_0_ref_0: &crate::map::IntoIter = &mut intoiter_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut bool_0: bool = false;
//     let mut bool_0_ref_0: &mut bool = &mut bool_0;
//     let mut str_0: &str = "yGvP";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
//     let mut str_1: &str = "KdCh51vhjyeSgctlW";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
//     let mut strread_1_ref_0: &crate::read::StrRead = &mut strread_1;
//     let mut f64_0: f64 = -2658.511418f64;
//     let mut n_0: number::N = crate::number::N::Float(f64_0);
//     let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
//     let mut number_0_ref_0: &crate::number::Number = &mut number_0;
//     let mut u8_0: u8 = 117u8;
//     let mut isize_0: isize = 10176isize;
//     let mut u8_1: u8 = 50u8;
//     let mut isize_1: isize = 6977isize;
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::FormFeed;
//     let mut bool_1: bool = crate::number::Number::is_u64(number_0_ref_0);
//     let mut position_0: crate::read::Position = crate::read::StrRead::position(strread_1_ref_0);
//     crate::read::StrRead::set_failed(strread_0_ref_0, bool_0_ref_0);
//     let mut charescape_1: ser::CharEscape = crate::ser::CharEscape::FormFeed;
//     let mut iter_0: crate::map::Iter = crate::map::Map::into_iter(map_1_ref_0);
//     let mut charescape_2: ser::CharEscape = crate::ser::CharEscape::Quote;
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::IntoIter::size_hint(intoiter_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_27() {
//     rusty_monitor::set_test_id(27);
//     let mut str_0: &str = "AU2OMm9NG2sKnFwI6j";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &crate::read::StrRead = &mut strread_0;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut i64_0: i64 = -7604i64;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut i64_1: i64 = -16171i64;
//     let mut n_0: number::N = crate::number::N::NegInt(i64_1);
//     let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
//     let mut number_0_ref_0: &crate::number::Number = &mut number_0;
//     let mut bool_0: bool = false;
//     let mut bool_0_ref_0: &mut bool = &mut bool_0;
//     let mut str_1: &str = "L7xJH9iAIuHHU";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
//     let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
//     let mut u64_0: u64 = 1908u64;
//     let mut n_1: number::N = crate::number::N::PosInt(u64_0);
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::LineFeed;
//     let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
//     crate::read::StrRead::set_failed(strread_1_ref_0, bool_0_ref_0);
//     let mut option_0: std::option::Option<u128> = crate::number::Number::as_u128(number_0_ref_0);
//     crate::map::Map::clear(map_1_ref_0);
//     let mut number_1_ref_0: &crate::number::Number = &mut number_1;
//     let mut bool_1: bool = crate::number::Number::is_i64(number_1_ref_0);
//     let mut n_2: number::N = crate::number::N::NegInt(i64_0);
//     let mut iter_0: crate::map::Iter = crate::map::Map::into_iter(map_0_ref_0);
//     let mut position_0: crate::read::Position = crate::read::StrRead::position(strread_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut category_0: error::Category = crate::error::Category::Io;
    let mut category_0_ref_0: &error::Category = &mut category_0;
    let mut category_1: error::Category = crate::error::Category::Io;
    let mut category_1_ref_0: &error::Category = &mut category_1;
    let mut isize_0: isize = 12405isize;
    let mut isize_0_ref_0: &isize = &mut isize_0;
    let mut str_0: &str = "UDisztcHJg29BtuNvcs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut usize_0: usize = 7706usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut str_1: &str = "rApYWgPOQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
    let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
    let mut usize_1: usize = 2310usize;
    let mut u64_0: u64 = 849u64;
    let mut n_0: number::N = crate::number::N::PosInt(u64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut usize_2: usize = 1319usize;
    let mut option_0: std::option::Option<i128> = crate::number::Number::as_i128(number_0_ref_0);
    let mut i128_0: i128 = std::option::Option::unwrap(option_0);
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    crate::read::StrRead::discard(strread_1_ref_0);
    crate::map::Map::sort_keys(map_0_ref_0);
    crate::read::StrRead::discard(strread_0_ref_0);
    let mut reference_0: read::Reference<isize> = crate::read::Reference::Copied(isize_0_ref_0);
    let mut bool_0: bool = crate::error::Category::eq(category_1_ref_0, category_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut str_0: &str = "19km2Zsry";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut usize_0: usize = 3776usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut value_0: value::Value = crate::value::Value::Object(map_0);
    let mut f64_0: f64 = 11715.987696f64;
    let mut usize_1: usize = 6701usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut u64_0: u64 = 1320u64;
    let mut n_0: number::N = crate::number::N::PosInt(u64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut value_1: value::Value = crate::value::Value::Number(number_0);
    let mut f64_1: f64 = -9232.433534f64;
    let mut n_1: number::N = crate::number::N::Float(f64_1);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut usize_2: usize = 7047usize;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_2);
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_2_ref_0);
    let mut keys_0_ref_0: &mut crate::map::Keys = &mut keys_0;
    let mut i64_0: i64 = 8663i64;
    let mut n_2: number::N = crate::number::N::NegInt(i64_0);
    let mut option_0: std::option::Option<&std::string::String> = crate::map::Keys::next(keys_0_ref_0);
    let mut string_0: &std::string::String = std::option::Option::unwrap(option_0);
    let mut n_2_ref_0: &number::N = &mut n_2;
    let mut n_3: number::N = crate::number::N::clone(n_2_ref_0);
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::clone(map_1_ref_0);
    let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_3);
    let mut option_1: std::option::Option<crate::number::Number> = crate::number::Number::from_f64(f64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut u64_0: u64 = 6015u64;
    let mut n_0: number::N = crate::number::N::PosInt(u64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut usize_0: usize = 3931usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_0);
    let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut intoiter_1: crate::map::IntoIter = crate::map::Map::into_iter(map_1);
    let mut intoiter_1_ref_0: &mut crate::map::IntoIter = &mut intoiter_1;
    let mut f64_0: f64 = -739.420584f64;
    let mut n_1: number::N = crate::number::N::Float(f64_0);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut prettyformatter_0: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::default();
    let mut prettyformatter_0_ref_0: &crate::ser::PrettyFormatter = &mut prettyformatter_0;
    let mut str_0: &str = "L1PPEM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &crate::read::StrRead = &mut strread_0;
    let mut usize_1: usize = 2610usize;
    let mut value_0: value::Value = crate::value::Value::Null;
    let mut category_0: error::Category = crate::error::Category::Data;
    let mut position_0: crate::read::Position = crate::read::StrRead::position(strread_0_ref_0);
    let mut category_0_ref_0: &error::Category = &mut category_0;
    let mut prettyformatter_1: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::clone(prettyformatter_0_ref_0);
    let mut option_0: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_1_ref_0);
    let mut option_1: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_0_ref_0);
    let mut option_2: std::option::Option<u64> = crate::number::Number::as_u64(number_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut i64_0: i64 = 5152i64;
    let mut n_0: number::N = crate::number::N::NegInt(i64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut usize_0: usize = 1642usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut usize_1: usize = 8301usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut usize_2: usize = 3562usize;
    let mut isize_0: isize = -5712isize;
    let mut isize_0_ref_0: &isize = &mut isize_0;
    let mut str_0: &str = "nH6LVp9X2V";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut f64_0: f64 = 6886.511822f64;
    let mut n_1: number::N = crate::number::N::Float(f64_0);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut option_0: std::option::Option<u64> = crate::number::Number::as_u64(number_1_ref_0);
    crate::read::StrRead::next(strread_0_ref_0);
    let mut reference_0: read::Reference<isize> = crate::read::Reference::Copied(isize_0_ref_0);
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_2);
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_2_ref_0);
    crate::map::Map::append(map_1_ref_0, map_0_ref_0);
    let mut iter_0_ref_0: &crate::map::Iter = &mut iter_0;
    let mut usize_3: usize = crate::map::Iter::len(iter_0_ref_0);
    let mut reference_0_ref_0: &read::Reference<isize> = &mut reference_0;
    let mut isize_1: &isize = crate::read::Reference::deref(reference_0_ref_0);
    let mut option_1: std::option::Option<f32> = crate::number::Number::as_f32(number_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut i64_0: i64 = -1642i64;
    let mut n_0: number::N = crate::number::N::NegInt(i64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut value_0: value::Value = crate::value::Value::Number(number_0);
    let mut category_0: error::Category = crate::error::Category::Syntax;
    let mut category_0_ref_0: &error::Category = &mut category_0;
    let mut category_1: error::Category = crate::error::Category::Data;
    let mut category_1_ref_0: &error::Category = &mut category_1;
    let mut i64_1: i64 = 9768i64;
    let mut n_1: number::N = crate::number::N::NegInt(i64_1);
    let mut n_1_ref_0: &number::N = &mut n_1;
    let mut f64_0: f64 = 1934.304455f64;
    let mut n_2: number::N = crate::number::N::Float(f64_0);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_2};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut str_0: &str = "UsWEpC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 5202usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut category_2: error::Category = crate::error::Category::Data;
    let mut category_2_ref_0: &error::Category = &mut category_2;
    let mut f64_1: f64 = 11003.360841f64;
    let mut n_3: number::N = crate::number::N::Float(f64_1);
    let mut number_2: crate::number::Number = crate::number::Number {n: n_3};
    let mut number_2_ref_0: &crate::number::Number = &mut number_2;
    let mut bool_0: bool = crate::number::Number::is_i64(number_2_ref_0);
    let mut tuple_0: () = crate::error::Category::assert_receiver_is_total_eq(category_2_ref_0);
    crate::map::Map::clear(map_0_ref_0);
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut option_0: std::option::Option<i64> = crate::number::Number::as_i64(number_1_ref_0);
    let mut n_4: number::N = crate::number::N::clone(n_1_ref_0);
    let mut bool_1: bool = crate::error::Category::eq(category_1_ref_0, category_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_41() {
//     rusty_monitor::set_test_id(41);
//     let mut str_0: &str = "tcrUR3LwjEa5DScam";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &crate::read::StrRead = &mut strread_0;
//     let mut str_1: &str = "";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut isize_0: isize = -362isize;
//     let mut isize_0_ref_0: &isize = &mut isize_0;
//     let mut category_0: error::Category = crate::error::Category::Data;
//     let mut category_0_ref_0: &error::Category = &mut category_0;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut usize_0: usize = 6924usize;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::iter_mut(map_1_ref_0);
//     let mut itermut_0_ref_0: &mut crate::map::IterMut = &mut itermut_0;
//     let mut bool_0: bool = false;
//     let mut bool_0_ref_0: &mut bool = &mut bool_0;
//     let mut i64_0: i64 = -6447i64;
//     let mut n_0: number::N = crate::number::N::NegInt(i64_0);
//     let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
//     let mut number_0_ref_0: &crate::number::Number = &mut number_0;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut option_0: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next_back(itermut_0_ref_0);
//     let mut iter_0: crate::map::Iter = crate::map::Map::into_iter(map_0_ref_0);
//     let mut category_1: error::Category = crate::error::Category::clone(category_0_ref_0);
//     let mut reference_0: read::Reference<isize> = crate::read::Reference::Copied(isize_0_ref_0);
//     let mut map_2_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     crate::map::Map::sort_keys(map_2_ref_0);
//     crate::de::from_str(str_1_ref_0);
//     let mut position_0: crate::read::Position = crate::read::StrRead::peek_position(strread_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut values_0: crate::map::Values = crate::map::Map::values(map_0_ref_0);
    let mut values_0_ref_0: &mut crate::map::Values = &mut values_0;
    let mut str_0: &str = "eG";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut str_1: &str = "ApqOn7HFQVr";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_1);
    let mut intovalues_0_ref_0: &crate::map::IntoValues = &mut intovalues_0;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut values_1: crate::map::Values = crate::map::Map::values(map_2_ref_0);
    let mut values_1_ref_0: &mut crate::map::Values = &mut values_1;
    let mut i64_0: i64 = -2836i64;
    let mut n_0: number::N = crate::number::N::NegInt(i64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::Tab;
    let mut prettyformatter_0: crate::ser::PrettyFormatter = crate::ser::PrettyFormatter::default();
    let mut tuple_0: () = crate::number::Number::assert_receiver_is_total_eq(number_0_ref_0);
    let mut option_0: std::option::Option<&value::Value> = crate::map::Values::next(values_1_ref_0);
    let mut charescape_1: ser::CharEscape = crate::ser::CharEscape::FormFeed;
    let mut charescape_2: ser::CharEscape = crate::ser::CharEscape::Tab;
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::IntoValues::size_hint(intovalues_0_ref_0);
    let mut deserializer_0: crate::de::Deserializer<crate::read::StrRead> = crate::de::Deserializer::from_str(str_1_ref_0);
    crate::read::StrRead::ignore_str(strread_0_ref_0);
    let mut charescape_3: ser::CharEscape = crate::ser::CharEscape::FormFeed;
    let mut option_1: std::option::Option<&value::Value> = crate::map::Values::next(values_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut u64_0: u64 = 4060u64;
    let mut n_0: number::N = crate::number::N::PosInt(u64_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut i64_0: i64 = 3167i64;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut itermut_0: crate::map::IterMut = crate::map::Map::iter_mut(map_0_ref_0);
    let mut itermut_0_ref_0: &mut crate::map::IterMut = &mut itermut_0;
    let mut u64_1: u64 = 7754u64;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_1);
    let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
    let mut i64_1: i64 = -13925i64;
    let mut n_1: number::N = crate::number::N::NegInt(i64_1);
    let mut number_1: crate::number::Number = crate::number::Number {n: n_1};
    let mut number_1_ref_0: &crate::number::Number = &mut number_1;
    let mut f64_0: f64 = 2451.690555f64;
    let mut n_2: number::N = crate::number::N::Float(f64_0);
    let mut number_2: crate::number::Number = crate::number::Number {n: n_2};
    let mut number_2_ref_0: &crate::number::Number = &mut number_2;
    let mut f32_0: f32 = 6747.059602f32;
    let mut option_0: std::option::Option<crate::number::Number> = crate::number::Number::from_f32(f32_0);
    let mut bool_0: bool = crate::number::Number::eq(number_2_ref_0, number_1_ref_0);
    let mut number_3: crate::number::Number = std::option::Option::unwrap(option_0);
    let mut option_1: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next(intoiter_0_ref_0);
    let mut n_3: number::N = crate::number::N::PosInt(u64_1);
    let mut tuple_0: (std::string::String, value::Value) = std::option::Option::unwrap(option_1);
    let mut option_2: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next_back(itermut_0_ref_0);
    let mut n_4: number::N = crate::number::N::NegInt(i64_0);
    let mut number_3_ref_0: &crate::number::Number = &mut number_3;
    let mut bool_1: bool = crate::number::Number::eq(number_3_ref_0, number_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut i64_0: i64 = 7163i64;
    let mut u8_0: u8 = 101u8;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_0);
    let mut intovalues_0_ref_0: &crate::map::IntoValues = &mut intovalues_0;
    let mut usize_0: usize = 7005usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_1);
    let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
    let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut intoiter_1: crate::map::IntoIter = crate::map::Map::into_iter(map_3);
    let mut intoiter_1_ref_0: &mut crate::map::IntoIter = &mut intoiter_1;
    let mut bool_0: bool = false;
    let mut i64_1: i64 = 9041i64;
    let mut n_0: number::N = crate::number::N::NegInt(i64_1);
    let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_4_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_4;
    let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_4_ref_0);
    let mut number_0: crate::number::Number = crate::number::Number {n: n_0};
    let mut value_0: value::Value = crate::value::Value::Bool(bool_0);
    let mut option_0: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_1_ref_0);
    let mut value_1: value::Value = crate::value::Value::Object(map_2);
    let mut number_0_ref_0: &crate::number::Number = &mut number_0;
    let mut iter_0_ref_0: &crate::map::Iter = &mut iter_0;
    let mut usize_1: usize = crate::map::Iter::len(iter_0_ref_0);
    let mut option_1: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next(intoiter_0_ref_0);
    let mut usize_2: usize = crate::map::IntoValues::len(intovalues_0_ref_0);
    let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::AsciiControl(u8_0);
    let mut n_1: number::N = crate::number::N::NegInt(i64_0);
    panic!("From RustyUnit with love");
}
}