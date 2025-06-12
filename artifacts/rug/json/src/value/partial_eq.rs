use super::Value;
use alloc::string::String;

fn eq_i64(value: &Value, other: i64) -> bool {
    value.as_i64() == Some(other)
}

fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64() == Some(other)
}

fn eq_f32(value: &Value, other: f32) -> bool {
    match value {
        Value::Number(n) => n.as_f32() == Some(other),
        _ => false,
    }
}

fn eq_f64(value: &Value, other: f64) -> bool {
    value.as_f64() == Some(other)
}

fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool() == Some(other)
}

fn eq_str(value: &Value, other: &str) -> bool {
    value.as_str() == Some(other)
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self)
    }
}

impl PartialEq<Value> for &str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, *self)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
    }
}

macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a mut Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    eq_i64[i8 i16 i32 i64 isize]
    eq_u64[u8 u16 u32 u64 usize]
    eq_f32[f32]
    eq_f64[f64]
    eq_bool[bool]
}

#[cfg(test)]
mod tests_llm_16_944 {
    use crate::value::Value;

    #[test]
    fn test_eq_with_same_f32_values() {
        let value1 = Value::from(1.0f32);
        let value2 = Value::from(1.0f32);
        assert!(value1.eq(&value2));
    }

    #[test]
    fn test_eq_with_different_f32_values() {
        let value1 = Value::from(2.0f32);
        let value2 = Value::from(3.0f32);
        assert!(!value1.eq(&value2));
    }

    #[test]
    fn test_eq_with_same_null_values() {
        let value1 = Value::Null;
        let value2 = Value::Null;
        assert!(value1.eq(&value2));
    }

    #[test]
    fn test_eq_with_different_types() {
        let value1 = Value::from(1.0f32);
        let value2 = Value::from("1.0");
        assert!(!value1.eq(&value2));
    }

    #[test]
    fn test_eq_with_different_f32_and_null() {
        let value1 = Value::from(1.0f32);
        let value2 = Value::Null;
        assert!(!value1.eq(&value2));
    }

    #[test]
    fn test_eq_with_same_string_values() {
        let value1 = Value::from("test");
        let value2 = Value::from("test");
        assert!(value1.eq(&value2));
    }

    #[test]
    fn test_eq_with_different_string_values() {
        let value1 = Value::from("foo");
        let value2 = Value::from("bar");
        assert!(!value1.eq(&value2));
    }
}

#[cfg(test)]
mod tests_llm_16_954 {
    use super::*;

use crate::*;
    use crate::{json, Value};

    #[test]
    fn test_eq_for_values() {
        let value1 = json!({"key": "value"});
        let value2 = json!({"key": "value"});
        let value3 = json!({"key": "different_value"});
        let value4 = json!(null);

        assert!(value1.eq(&value2));
        assert!(!value1.eq(&value3));
        assert!(!value1.eq(&value4));
        assert!(value4.eq(&Value::Null));
    }

    #[test]
    fn test_eq_for_numbers() {
        let number1 = Value::Number(Number::from(42));
        let number2 = Value::Number(Number::from(42));
        let number3 = Value::Number(Number::from(24));
        
        assert!(number1.eq(&number2));
        assert!(!number1.eq(&number3));
    }

    #[test]
    fn test_eq_for_strings() {
        let string1 = Value::String("Hello".into());
        let string2 = Value::String("Hello".into());
        let string3 = Value::String("World".into());
        
        assert!(string1.eq(&string2));
        assert!(!string1.eq(&string3));
    }

    #[test]
    fn test_eq_for_bools() {
        let true_value = Value::Bool(true);
        let false_value = Value::Bool(false);
        
        assert!(true_value.eq(&Value::Bool(true)));
        assert!(!true_value.eq(&false_value));
    }

    #[test]
    fn test_eq_for_null() {
        let null_value = Value::Null;
        assert!(null_value.eq(&Value::Null));
        assert!(!null_value.eq(&Value::Bool(false)));
    }
}

#[cfg(test)]
mod tests_llm_16_980 {
    use super::*;

use crate::*;
    use crate::{json, Value};

    #[test]
    fn test_eq_value_with_value() {
        let value1 = json!(42);
        let value2 = json!(42);
        let value3 = json!("42");

        assert!(value1 == value2);
        assert!(value1 != value3);
    }

    #[test]
    fn test_eq_value_with_string() {
        let value = json!("test");
        let string1 = String::from("test");
        let string2 = String::from("not test");

        assert!(value == string1);
        assert!(value != string2);
    }

    #[test]
    fn test_eq_value_with_number() {
        let value1 = json!(1);
        let value2 = json!(1.0);
        let value3 = json!(2);

        assert!(value1 == value2);
        assert!(value1 != value3);
    }

    #[test]
    fn test_eq_value_with_boolean() {
        let value1 = json!(true);
        let value2 = json!(true);
        let value3 = json!(false);

        assert!(value1 == value2);
        assert!(value1 != value3);
    }

    #[test]
    fn test_eq_value_with_null() {
        let value1 = json!(null);
        let value2 = json!(null);
        let value3 = json!(false);

        assert!(value1 == value2);
        assert!(value1 != value3);
    }

    #[test]
    fn test_eq_value_with_array() {
        let value1 = json!([1, 2, 3]);
        let value2 = json!([1, 2, 3]);
        let value3 = json!([1, 2, 4]);

        assert!(value1 == value2);
        assert!(value1 != value3);
    }

    #[test]
    fn test_eq_value_with_object() {
        let value1 = json!({"key": "value"});
        let value2 = json!({"key": "value"});
        let value3 = json!({"key": "different value"});

        assert!(value1 == value2);
        assert!(value1 != value3);
    }
}

#[cfg(test)]
mod tests_llm_16_982 {
    use super::*;

use crate::*;
    use crate::{json, Value};

    #[test]
    fn test_eq_with_string() {
        let string_value = String::from("test");
        let json_value = json!("test");

        assert!(string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_different_string() {
        let string_value = String::from("test");
        let json_value = json!("different");

        assert!(!string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_null() {
        let string_value = String::from("test");
        let json_value = json!(null);

        assert!(!string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_bool() {
        let string_value = String::from("true");
        let json_value = json!(true);

        assert!(!string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_number() {
        let string_value = String::from("123");
        let json_value = json!(123);

        assert!(!string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_number_as_string() {
        let string_value = String::from("123");
        let json_value = json!("123");

        assert!(string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_json_array() {
        let string_value = String::from("test");
        let json_value = json!(["test", "different"]);

        assert!(!string_value.eq(&json_value));
    }

    #[test]
    fn test_eq_with_json_object() {
        let string_value = String::from("test");
        let json_value = json!({"key": "test"});

        assert!(!string_value.eq(&json_value));
    }
}

#[cfg(test)]
mod tests_llm_16_995 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_eq_with_u8() {
        let value: Value = Value::from(10u8);
        let equal = (10u8).eq(&value);
        assert!(equal);
    }

    #[test]
    fn test_eq_with_different_u8() {
        let value: Value = Value::from(10u8);
        let equal = (5u8).eq(&value);
        assert!(!equal);
    }

    #[test]
    fn test_eq_with_number() {
        let value: Value = Value::from(10);
        let equal = (10u8).eq(&value);
        assert!(equal);
    }

    #[test]
    fn test_eq_with_float() {
        let value: Value = Value::from(10.0);
        let equal = (10u8).eq(&value);
        assert!(!equal);
    }

    #[test]
    fn test_eq_with_string() {
        let value: Value = Value::from("10");
        let equal = (10u8).eq(&value);
        assert!(!equal);
    }

    #[test]
    fn test_eq_with_null() {
        let value: Value = Value::Null;
        let equal = (10u8).eq(&value);
        assert!(!equal);
    }
}

#[cfg(test)]
mod tests_llm_16_997 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_eq_bool() {
        assert!(eq_bool(&Value::Bool(true), true));
        assert!(!eq_bool(&Value::Bool(true), false));
        assert!(eq_bool(&Value::Bool(false), false));
        assert!(!eq_bool(&Value::Bool(false), true));
        assert!(eq_bool(&Value::Null, false));
        assert!(eq_bool(&Value::Null, true));
    }
}

#[cfg(test)]
mod tests_llm_16_998 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_eq_f32() {
        let number_f32 = Value::Number(Number::from_f32(12.34).unwrap());
        let number_f32_diff = Value::Number(Number::from_f32(12.35).unwrap());
        let number_i32 = Value::Number(Number::from(12));
        let number_nan = Value::Number(Number::from_f32(f32::NAN).unwrap());
        let value_null = Value::Null;

        assert!(eq_f32(&number_f32, 12.34));
        assert!(!eq_f32(&number_f32, 12.35));
        assert!(!eq_f32(&number_f32, 12.0));
        assert!(!eq_f32(&number_i32, 12.34));
        assert!(!eq_f32(&number_nan, 12.34));
        assert!(!eq_f32(&value_null, 12.34));
    }
}

#[cfg(test)]
mod tests_llm_16_1002 {
    use super::*;

use crate::*;
    use crate::Value;

    #[test]
    fn test_eq_u64() {
        // Test with a Value that represents a u64 equal to the number
        let value_eq = Value::Number(Number::from(42u64));
        assert!(eq_u64(&value_eq, 42));

        // Test with a Value that represents a u64 not equal to the number
        let value_neq = Value::Number(Number::from(43u64));
        assert!(!eq_u64(&value_neq, 42));

        // Test with a Value that represents a number not convertible to u64
        let value_float = Value::Number(Number::from_f64(42.0).unwrap());
        assert!(!eq_u64(&value_float, 42));

        // Test with a Value that represents a different type (Boolean)
        let value_bool = Value::Bool(true);
        assert!(!eq_u64(&value_bool, 1));
    }
}
