use super::Value;
use crate::map::Map;
use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::{self, Display};
use core::ops;

/// A type that can be used to index into a `serde_json::Value`.
///
/// The [`get`] and [`get_mut`] methods of `Value` accept any type that
/// implements `Index`, as does the [square-bracket indexing operator]. This
/// trait is implemented for strings which are used as the index into a JSON
/// map, and for `usize` which is used as the index into a JSON array.
///
/// [`get`]: Value::get
/// [`get_mut`]: Value::get_mut
/// [square-bracket indexing operator]: Value#impl-Index%3CI%3E-for-Value
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json`.
///
/// # Examples
///
/// ```
/// # use serde_json::json;
/// #
/// let data = json!({ "inner": [1, 2, 3] });
///
/// // Data is a JSON map so it can be indexed with a string.
/// let inner = &data["inner"];
///
/// // Inner is a JSON array so it can be indexed with an integer.
/// let first = &inner[0];
///
/// assert_eq!(first, 1);
/// ```
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;

    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;

    /// Panic if array index out of bounds. If key is not already in the object,
    /// insert it with a value of null. Panic if Value is a type that cannot be
    /// indexed into, except if Value is null then it can be treated as an empty
    /// object.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        match v {
            Value::Array(vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of JSON array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of JSON {}", self, Type(v)),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Object(map) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Object(map) => map.get_mut(self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = v {
            *v = Value::Object(Map::new());
        }
        match v {
            Value::Object(map) => map.entry(self.to_owned()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in JSON {}", self, Type(v)),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }
}

impl<T> Index for &T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for alloc::string::String {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}

/// Used in panic messages.
struct Type<'a>(&'a Value);

impl<'a> Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Value::Null => formatter.write_str("null"),
            Value::Bool(_) => formatter.write_str("boolean"),
            Value::Number(_) => formatter.write_str("number"),
            Value::String(_) => formatter.write_str("string"),
            Value::Array(_) => formatter.write_str("array"),
            Value::Object(_) => formatter.write_str("object"),
        }
    }
}

// The usual semantics of Index is to panic on invalid indexing.
//
// That said, the usual semantics are for things like Vec and BTreeMap which
// have different use cases than Value. If you are working with a Vec, you know
// that you are working with a Vec and you can get the len of the Vec and make
// sure your indices are within bounds. The Value use cases are more
// loosey-goosey. You got some JSON from an endpoint and you want to pull values
// out of it. Outside of this Index impl, you already have the option of using
// value.as_array() and working with the Vec directly, or matching on
// Value::Array and getting the Vec directly. The Index impl means you can skip
// that and index directly into the thing using a concise syntax. You don't have
// to check the type, you don't have to check the len, it is all about what you
// expect the Value to look like.
//
// Basically the use cases that would be well served by panicking here are
// better served by using one of the other approaches: get and get_mut,
// as_array, or match. The value of this impl is that it adds a way of working
// with Value that is not well served by the existing approaches: concise and
// careless and sometimes that is exactly what you want.
impl<I> ops::Index<I> for Value
where
    I: Index,
{
    type Output = Value;

    /// Index into a `serde_json::Value` using the syntax `value[0]` or
    /// `value["k"]`.
    ///
    /// Returns `Value::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is an array
    /// or a number. Also returns `Value::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the array.
    ///
    /// For retrieving deeply nested values, you should have a look at the
    /// `Value::pointer` method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let data = json!({
    ///     "x": {
    ///         "y": ["z", "zz"]
    ///     }
    /// });
    ///
    /// assert_eq!(data["x"]["y"], json!(["z", "zz"]));
    /// assert_eq!(data["x"]["y"][0], json!("z"));
    ///
    /// assert_eq!(data["a"], json!(null)); // returns null for undefined values
    /// assert_eq!(data["a"]["b"], json!(null)); // does not panic
    /// ```
    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Value
where
    I: Index,
{
    /// Write into a `serde_json::Value` using the syntax `value[0] = ...` or
    /// `value["k"] = ...`.
    ///
    /// If the index is a number, the value must be an array of length bigger
    /// than the index. Indexing into a value that is not an array or an array
    /// that is too small will panic.
    ///
    /// If the index is a string, the value must be an object or null which is
    /// treated like an empty object. If the key is not already present in the
    /// object, it will be inserted with a value of null. Indexing into a value
    /// that is neither an object nor null will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut data = json!({ "x": 0 });
    ///
    /// // replace an existing key
    /// data["x"] = json!(1);
    ///
    /// // insert a new key
    /// data["y"] = json!([false, false, false]);
    ///
    /// // replace an array value
    /// data["y"][0] = json!(true);
    ///
    /// // inserted a deeply nested key
    /// data["a"]["b"]["c"]["d"] = json!(true);
    ///
    /// println!("{}", data);
    /// ```
    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }
}

#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_index_into() {
        // Create a Value that is a JSON object
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": {
                "nestedKey": "nestedValue"
            }
        });

        // Test indexing into the JSON object
        let key1_result = "key1".index_into(&value);
        let key2_result = "key2".index_into(&value);
        let key3_result = "key3".index_into(&value);
        let missing_key_result = "missingKey".index_into(&value);
        
        assert_eq!(key1_result, Some(&json!("value1")));
        assert_eq!(key2_result, Some(&json!("value2")));
        assert_eq!(key3_result, Some(&json!({
            "nestedKey": "nestedValue"
        })));
        assert_eq!(missing_key_result, None);
    }

    #[test]
    fn test_index_into_array() {
        // Create a Value that is a JSON array
        let value = json!(["A", "B", "C"]);

        // Test indexing into the JSON array
        let first_result = 0.index_into(&value);
        let second_result = 1.index_into(&value);
        let out_of_bounds_result = 3.index_into(&value);
        
        assert_eq!(first_result, Some(&json!("A")));
        assert_eq!(second_result, Some(&json!("B")));
        assert_eq!(out_of_bounds_result, None);
    }

    #[test]
    fn test_index_into_with_non_string_index() {
        let value = json!({
            "key1": "value1"
        });

        // Test with a non-string index (should return None)
        let non_string_index_result = 1.index_into(&value);
        assert_eq!(non_string_index_result, None);
    }
}

#[cfg(test)]
mod tests_llm_16_123 {
    use super::*;

use crate::*;
    use crate::{json, Value};

    #[test]
    fn test_index_into() {
        let json_value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": {
                "nested_key": "nested_value"
            }
        });

        let key = String::from("key1");
        let result = key.index_into(&json_value);
        assert_eq!(result, Some(&json!("value1")));

        let key = String::from("key2");
        let result = key.index_into(&json_value);
        assert_eq!(result, Some(&json!("value2")));

        let key = String::from("key3");
        let result = key.index_into(&json_value);
        assert_eq!(result, Some(&json!({"nested_key": "nested_value"})));

        let key = String::from("key4");
        let result = key.index_into(&json_value);
        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod tests_llm_16_364 {
    use super::*;

use crate::*;
    use crate::{json, Value};

    #[test]
    fn test_index_into_valid_key() {
        let key = "key";
        let value = json!({ "key": "value" });
        let result = key.index_into(&value);
        assert_eq!(result, Some(&json!("value")));
    }

    #[test]
    fn test_index_into_non_existent_key() {
        let key = "non_existent_key";
        let value = json!({ "key": "value" });
        let result = key.index_into(&value);
        assert_eq!(result, None);
    }

    #[test]
    fn test_index_into_with_non_object() {
        let key = "key";
        let value = json!([1, 2, 3]);
        let result = key.index_into(&value);
        assert_eq!(result, None);
    }

    #[test]
    fn test_index_into_with_null() {
        let key = "key";
        let value = json!(null);
        let result = key.index_into(&value);
        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod tests_llm_16_365 {
    use super::*;

use crate::*;
    use crate::{json, Value};

    #[test]
    fn test_index_into_mut_existing_key() {
        let mut value = json!({ "key": "initial value" });
        let key = String::from("key");
        if let Some(val) = key.index_into_mut(&mut value) {
            *val = json!("updated value");
        }
        assert_eq!(value, json!({ "key": "updated value" }));
    }

    #[test]
    fn test_index_into_mut_non_existing_key() {
        let mut value = json!({ "key": "initial value" });
        let key = String::from("non_existing_key");
        let result = key.index_into_mut(&mut value);
        assert!(result.is_none());
    }

    #[test]
    fn test_index_into_mut_non_object_value() {
        let mut value = json!(42);
        let key = String::from("key");
        let result = key.index_into_mut(&mut value);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_369 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_index_or_insert_existing_index() {
        let mut value = json!(["first", "second", "third"]);
        let index = 1;
        let result = index.index_or_insert(&mut value);
        assert_eq!(result, &mut json!("second"));
    }

    #[test]
    #[should_panic(expected = "cannot access index 3 of JSON array of length 3")]
    fn test_index_or_insert_out_of_bounds() {
        let mut value = json!(["first", "second", "third"]);
        let index = 3;
        index.index_or_insert(&mut value);
    }

    #[test]
    #[should_panic(expected = "cannot access index 0 of JSON {}")]
    fn test_index_or_insert_non_array() {
        let mut value = json!({});
        let index = 0;
        index.index_or_insert(&mut value);
    }
}

#[cfg(test)]
mod tests_llm_16_937 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_index_mut_insert() {
        let mut value = json!({});
        value["key"] = json!(42);
        assert_eq!(value["key"], json!(42));
    }

    #[test]
    fn test_index_mut_replace() {
        let mut value = json!({"key": 1});
        value["key"] = json!(2);
        assert_eq!(value["key"], json!(2));
    }

    #[test]
    fn test_index_mut_insert_new_key() {
        let mut value = json!({});
        value["new_key"] = json!("value");
        assert_eq!(value["new_key"], json!("value"));
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_mut_invalid_key() {
        let mut value = json!({"key": 1});
        let _ = value["invalid_key"].as_i64(); // This will panic
    }

    #[test]
    fn test_index_mut_array() {
        let mut value = json!([1, 2, 3]);
        value[1] = json!(42);
        assert_eq!(value[1], json!(42));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_mut_array_out_of_bounds() {
        let mut value = json!([1, 2]);
        value[2] = json!(3); // This will panic
    }

    #[test]
    fn test_index_mut_nested() {
        let mut value = json!({});
        value["outer"]["inner"] = json!(true);
        assert_eq!(value["outer"]["inner"], json!(true));
    }
}
