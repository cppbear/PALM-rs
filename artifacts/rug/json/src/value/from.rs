use super::Value;
use crate::map::Map;
use crate::number::Number;
use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Number(n.into())
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

#[cfg(feature = "arbitrary_precision")]
from_integer! {
    i128 u128
}

impl From<f32> for Value {
    /// Convert 32-bit floating point number to `Value::Number`, or
    /// `Value::Null` if infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f32 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f32) -> Self {
        Number::from_f32(f).map_or(Value::Null, Value::Number)
    }
}

impl From<f64> for Value {
    /// Convert 64-bit floating point number to `Value::Number`, or
    /// `Value::Null` if infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let f: f64 = 13.37;
    /// let x: Value = f.into();
    /// ```
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(Value::Null, Value::Number)
    }
}

impl From<bool> for Value {
    /// Convert boolean to `Value::Bool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let b = false;
    /// let x: Value = b.into();
    /// ```
    fn from(f: bool) -> Self {
        Value::Bool(f)
    }
}

impl From<String> for Value {
    /// Convert `String` to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: String = "lorem".to_owned();
    /// let x: Value = s.into();
    /// ```
    fn from(f: String) -> Self {
        Value::String(f)
    }
}

impl From<&str> for Value {
    /// Convert string slice to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let s: &str = "lorem";
    /// let x: Value = s.into();
    /// ```
    fn from(f: &str) -> Self {
        Value::String(f.to_owned())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    /// Convert copy-on-write string to `Value::String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("lorem");
    /// let x: Value = s.into();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Owned("lorem".to_owned());
    /// let x: Value = s.into();
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.into_owned())
    }
}

impl From<Number> for Value {
    /// Convert `Number` to `Value::Number`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Number, Value};
    ///
    /// let n = Number::from(7);
    /// let x: Value = n.into();
    /// ```
    fn from(f: Number) -> Self {
        Value::Number(f)
    }
}

impl From<Map<String, Value>> for Value {
    /// Convert map (with string keys) to `Value::Object`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{Map, Value};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_owned(), "ipsum".into());
    /// let x: Value = m.into();
    /// ```
    fn from(f: Map<String, Value>) -> Self {
        Value::Object(f)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    /// Convert a `Vec` to `Value::Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: Vec<T>) -> Self {
        Value::Array(f.into_iter().map(Into::into).collect())
    }
}

impl<T: Into<Value>, const N: usize> From<[T; N]> for Value {
    fn from(array: [T; N]) -> Self {
        Value::Array(array.into_iter().map(Into::into).collect())
    }
}

impl<T: Clone + Into<Value>> From<&[T]> for Value {
    /// Convert a slice to `Value::Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// ```
    fn from(f: &[T]) -> Self {
        Value::Array(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Value>> FromIterator<T> for Value {
    /// Create a `Value::Array` by collecting an iterator of array elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: Value = v.collect();
    /// ```
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into_iter().collect();
    /// ```
    ///
    /// ```
    /// use std::iter::FromIterator;
    /// use serde_json::Value;
    ///
    /// let x: Value = Value::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Array(iter.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<String>, V: Into<Value>> FromIterator<(K, V)> for Value {
    /// Create a `Value::Object` by collecting an iterator of key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec![("lorem", 40), ("ipsum", 2)];
    /// let x: Value = v.into_iter().collect();
    /// ```
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Object(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl From<()> for Value {
    /// Convert `()` to `Value::Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// let u = ();
    /// let x: Value = u.into();
    /// ```
    fn from((): ()) -> Self {
        Value::Null
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Value::Null,
            Some(value) => Into::into(value),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_912 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_unit() {
        let unit_value: Value = ().into();
        assert_eq!(unit_value, Value::Null);
    }
}

#[cfg(test)]
mod tests_llm_16_914 {
    use super::*; // Ensure to include the relevant module

use crate::*;
    use crate::Value;
    use std::borrow::Cow;

    #[test]
    fn test_cow_borrowed() {
        let s: Cow<str> = Cow::Borrowed("lorem");
        let x: Value = s.into();
        assert_eq!(x, Value::String("lorem".to_owned()));
    }

    #[test]
    fn test_cow_owned() {
        let s: Cow<str> = Cow::Owned("lorem".to_owned());
        let x: Value = s.into();
        assert_eq!(x, Value::String("lorem".to_owned()));
    }

    #[test]
    fn test_empty_cow() {
        let s: Cow<str> = Cow::Borrowed("");
        let x: Value = s.into();
        assert_eq!(x, Value::String("".to_owned()));
    }

    #[test]
    fn test_long_string_cow() {
        let s: Cow<str> = Cow::Owned("This is a very long string to test the from function.".to_owned());
        let x: Value = s.into();
        assert_eq!(x, Value::String("This is a very long string to test the from function.".to_owned()));
    }
}

#[cfg(test)]
mod tests_llm_16_915 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_string_to_value_conversion() {
        let s: String = "lorem".to_owned();
        let value: Value = Value::from(s.clone());
        assert_eq!(value, Value::String(s));
    }

    #[test]
    fn test_empty_string_to_value() {
        let s: String = "".to_owned();
        let value: Value = Value::from(s.clone());
        assert_eq!(value, Value::String(s));
    }

    #[test]
    fn test_special_characters_string_to_value() {
        let s: String = "lorem \u{1F600}".to_owned();
        let value: Value = Value::from(s.clone());
        assert_eq!(value, Value::String(s));
    }

    #[test]
    fn test_large_string_to_value() {
        let s: String = "a".repeat(1000);
        let value: Value = Value::from(s.clone());
        assert_eq!(value, Value::String(s));
    }

    #[test]
    fn test_string_with_numbers_to_value() {
        let s: String = "123".to_owned();
        let value: Value = Value::from(s.clone());
        assert_eq!(value, Value::String(s));
    }

    #[test]
    fn test_string_with_special_characters() {
        let s: String = "lorem, ipsum: dolor; sit".to_owned();
        let value: Value = Value::from(s.clone());
        assert_eq!(value, Value::String(s));
    }
}

#[cfg(test)]
mod tests_llm_16_917 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_bool_true() {
        let value: Value = true.into();
        match value {
            Value::Bool(b) => assert!(b),
            _ => panic!("Expected Value::Bool"),
        }
    }

    #[test]
    fn test_from_bool_false() {
        let value: Value = false.into();
        match value {
            Value::Bool(b) => assert!(!b),
            _ => panic!("Expected Value::Bool"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_919 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_f32_valid() {
        let f: f32 = 13.37;
        let value: Value = f.into();
        assert_eq!(value, Value::Number(Number::from_f32(13.37).unwrap()));
    }

    #[test]
    fn test_from_f32_negative() {
        let f: f32 = -7.5;
        let value: Value = f.into();
        assert_eq!(value, Value::Number(Number::from_f32(-7.5).unwrap()));
    }

    #[test]
    fn test_from_f32_nan() {
        let f: f32 = f32::NAN;
        let value: Value = f.into();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_from_f32_infinity() {
        let f: f32 = f32::INFINITY;
        let value: Value = f.into();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_from_f32_negative_infinity() {
        let f: f32 = f32::NEG_INFINITY;
        let value: Value = f.into();
        assert_eq!(value, Value::Null);
    }
}

#[cfg(test)]
mod tests_llm_16_920 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_f64_finite() {
        let input: f64 = 13.37;
        let value: Value = input.into();
        assert!(value.is_number());
        assert_eq!(value.as_f64(), Some(13.37));
    }

    #[test]
    fn test_from_f64_nan() {
        let input: f64 = f64::NAN;
        let value: Value = input.into();
        assert!(value.is_null());
    }

    #[test]
    fn test_from_f64_infinite() {
        let input: f64 = f64::INFINITY;
        let value: Value = input.into();
        assert!(value.is_null());
    }

    #[test]
    fn test_from_f64_negative_infinity() {
        let input: f64 = f64::NEG_INFINITY;
        let value: Value = input.into();
        assert!(value.is_null());
    }

    #[test]
    fn test_from_f64_zero() {
        let input: f64 = 0.0;
        let value: Value = input.into();
        assert!(value.is_number());
        assert_eq!(value.as_f64(), Some(0.0));
    }

    #[test]
    fn test_from_f64_negative_zero() {
        let input: f64 = -0.0;
        let value: Value = input.into();
        assert!(value.is_number());
        assert_eq!(value.as_f64(), Some(0.0)); // both +0.0 and -0.0 are represented as 0.0
    }
}

#[cfg(test)]
mod tests_llm_16_922 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_i32() {
        let value: Value = Value::from(42_i32);
        assert_eq!(value, Value::Number(crate::Number::from(42)));
    }

    #[test]
    fn test_from_negative_i32() {
        let value: Value = Value::from(-1_i32);
        assert_eq!(value, Value::Number(crate::Number::from(-1)));
    }
    
    #[test]
    fn test_from_zero_i32() {
        let value: Value = Value::from(0_i32);
        assert_eq!(value, Value::Number(crate::Number::from(0)));
    }
    
    #[test]
    fn test_from_i32_large() {
        let value: Value = Value::from(1_000_000_i32);
        assert_eq!(value, Value::Number(crate::Number::from(1_000_000)));
    }

    #[test]
    fn test_from_i32_min() {
        let value: Value = Value::from(i32::MIN);
        assert_eq!(value, Value::Number(crate::Number::from(i32::MIN)));
    }

    #[test]
    fn test_from_i32_max() {
        let value: Value = Value::from(i32::MAX);
        assert_eq!(value, Value::Number(crate::Number::from(i32::MAX)));
    }
}

#[cfg(test)]
mod tests_llm_16_923 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_i64_conversion() {
        let input: i64 = 42;
        let value: Value = Value::from(input);
        assert_eq!(value, Value::Number(Number::from(input)));
    }

    #[test]
    fn test_from_negative_i64_conversion() {
        let input: i64 = -42;
        let value: Value = Value::from(input);
        assert_eq!(value, Value::Number(Number::from(input)));
    }

    #[test]
    fn test_from_large_i64_conversion() {
        let input: i64 = 1_000_000_000;
        let value: Value = Value::from(input);
        assert_eq!(value, Value::Number(Number::from(input)));
    }

    #[test]
    fn test_from_zero_i64_conversion() {
        let input: i64 = 0;
        let value: Value = Value::from(input);
        assert_eq!(value, Value::Number(Number::from(input)));
    }
}

#[cfg(test)]
mod tests_llm_16_924 {
    use super::*;

use crate::*;
    use crate::{Value, Number};

    #[test]
    fn test_from_i8() {
        let num: i8 = 42;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(42)));
    }

    #[test]
    fn test_from_i8_negative() {
        let num: i8 = -42;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(-42)));
    }

    #[test]
    fn test_from_i8_zero() {
        let num: i8 = 0;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(0)));
    }

    #[test]
    fn test_from_u8() {
        let num: u8 = 255;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(255)));
    }

    #[test]
    fn test_from_u8_zero() {
        let num: u8 = 0;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(0)));
    }

    #[test]
    fn test_from_f32() {
        let num: f32 = 3.14;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from_f64(3.14).unwrap()));
    }

    #[test]
    fn test_from_f32_nan() {
        let num: f32 = f32::NAN;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_from_f32_infinity() {
        let num: f32 = f32::INFINITY;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_from_f64() {
        let num: f64 = 3.14;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from_f64(3.14).unwrap()));
    }

    #[test]
    fn test_from_f64_nan() {
        let num: f64 = f64::NAN;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_from_f64_infinity() {
        let num: f64 = f64::INFINITY;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Null);
    }
}

#[cfg(test)]
mod tests_llm_16_925 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_isize() {
        assert_eq!(Value::from(42_isize), Value::Number(crate::Number::from(42)));
        assert_eq!(Value::from(-42_isize), Value::Number(crate::Number::from(-42)));
        assert_eq!(Value::from(0_isize), Value::Number(crate::Number::from(0)));
    }

    #[test]
    fn test_from_negative_isize() {
        assert_eq!(Value::from(-1_isize), Value::Number(crate::Number::from(-1)));
    }

    #[test]
    fn test_from_large_isize() {
        let large_value: isize = isize::max_value(); // Maximum value for isize
        assert_eq!(Value::from(large_value), Value::Number(crate::Number::from(large_value)));
    }

    #[test]
    fn test_from_small_isize() {
        let small_value: isize = isize::min_value(); // Minimum value for isize
        assert_eq!(Value::from(small_value), Value::Number(crate::Number::from(small_value)));
    }
}

#[cfg(test)]
mod tests_llm_16_928 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_from_u16_to_value() {
        let num: u16 = 42;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }

    #[test]
    fn test_from_u16_zero() {
        let num: u16 = 0;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }

    #[test]
    fn test_from_u16_max_value() {
        let num: u16 = u16::MAX;
        let value: Value = Value::from(num);
        assert_eq!(value, Value::Number(Number::from(num)));
    }
}
