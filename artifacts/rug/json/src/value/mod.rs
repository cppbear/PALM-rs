//! The Value enum, a loosely typed way of representing any valid JSON value.
//!
//! # Constructing JSON
//!
//! Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
//! objects with very natural JSON syntax.
//!
//! ```
//! use serde_json::json;
//!
//! fn main() {
//!     // The type of `john` is `serde_json::Value`
//!     let john = json!({
//!         "name": "John Doe",
//!         "age": 43,
//!         "phones": [
//!             "+44 1234567",
//!             "+44 2345678"
//!         ]
//!     });
//!
//!     println!("first phone number: {}", john["phones"][0]);
//!
//!     // Convert to a string of JSON and print it out
//!     println!("{}", john.to_string());
//! }
//! ```
//!
//! The `Value::to_string()` function converts a `serde_json::Value` into a
//! `String` of JSON text.
//!
//! One neat thing about the `json!` macro is that variables and expressions can
//! be interpolated directly into the JSON value as you are building it. Serde
//! will check at compile time that the value you are interpolating is able to
//! be represented as JSON.
//!
//! ```
//! # use serde_json::json;
//! #
//! # fn random_phone() -> u16 { 0 }
//! #
//! let full_name = "John Doe";
//! let age_last_year = 42;
//!
//! // The type of `john` is `serde_json::Value`
//! let john = json!({
//!     "name": full_name,
//!     "age": age_last_year + 1,
//!     "phones": [
//!         format!("+44 {}", random_phone())
//!     ]
//! });
//! ```
//!
//! A string of JSON data can be parsed into a `serde_json::Value` by the
//! [`serde_json::from_str`][from_str] function. There is also
//! [`from_slice`][from_slice] for parsing from a byte slice `&[u8]` and
//! [`from_reader`][from_reader] for parsing from any `io::Read` like a File or
//! a TCP stream.
//!
//! ```
//! use serde_json::{json, Value, Error};
//!
//! fn untyped_example() -> Result<(), Error> {
//!     // Some JSON input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43,
//!             "phones": [
//!                 "+44 1234567",
//!                 "+44 2345678"
//!             ]
//!         }"#;
//!
//!     // Parse the string of data into serde_json::Value.
//!     let v: Value = serde_json::from_str(data)?;
//!
//!     // Access parts of the data by indexing with square brackets.
//!     println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//!
//!     Ok(())
//! }
//! #
//! # untyped_example().unwrap();
//! ```
//!
//! [macro]: crate::json
//! [from_str]: crate::de::from_str
//! [from_slice]: crate::de::from_slice
//! [from_reader]: crate::de::from_reader

use crate::error::Error;
use crate::io;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Display};
use core::mem;
use core::str;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub use self::index::Index;
pub use self::ser::Serializer;
pub use crate::map::Map;
pub use crate::number::Number;

#[cfg(feature = "raw_value")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw_value")))]
pub use crate::raw::{to_raw_value, RawValue};

/// Represents any valid JSON value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
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

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => Debug::fmt(number, formatter),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Array(vec) => {
                tri!(formatter.write_str("Array "));
                Debug::fmt(vec, formatter)
            }
            Value::Object(map) => {
                tri!(formatter.write_str("Object "));
                Debug::fmt(map, formatter)
            }
        }
    }
}

impl Display for Value {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let json = json!({ "city": "London", "street": "10 Downing Street" });
    ///
    /// // Compact format:
    /// //
    /// // {"city":"London","street":"10 Downing Street"}
    /// let compact = format!("{}", json);
    /// assert_eq!(compact,
    ///     "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
    ///
    /// // Pretty format:
    /// //
    /// // {
    /// //   "city": "London",
    /// //   "street": "10 Downing Street"
    /// // }
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty,
    ///     "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct WriterFormatter<'a, 'b: 'a> {
            inner: &'a mut fmt::Formatter<'b>,
        }

        impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                // Safety: the serializer below only emits valid utf8 when using
                // the default formatter.
                let s = unsafe { str::from_utf8_unchecked(buf) };
                tri!(self.inner.write_str(s).map_err(io_error));
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        fn io_error(_: fmt::Error) -> io::Error {
            // Error value does not matter because Display impl just maps it
            // back to fmt::Error.
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }

        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            super::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            super::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl Value {
    /// Index into a JSON array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let object = json!({ "A": 65, "B": 66, "C": 67 });
    /// assert_eq!(*object.get("A").unwrap(), json!(65));
    ///
    /// let array = json!([ "A", "B", "C" ]);
    /// assert_eq!(*array.get(2).unwrap(), json!("C"));
    ///
    /// assert_eq!(array.get("A"), None);
    /// ```
    ///
    /// Square brackets can also be used to index into a value in a more concise
    /// way. This returns `Value::Null` in cases where `get` would have returned
    /// `None`.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let object = json!({
    ///     "A": ["a", "á", "à"],
    ///     "B": ["b", "b́"],
    ///     "C": ["c", "ć", "ć̣", "ḉ"],
    /// });
    /// assert_eq!(object["B"][0], json!("b"));
    ///
    /// assert_eq!(object["D"], json!(null));
    /// assert_eq!(object[0]["x"]["y"]["z"], json!(null));
    /// ```
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    /// Mutably index into a JSON array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut object = json!({ "A": 65, "B": 66, "C": 67 });
    /// *object.get_mut("A").unwrap() = json!(69);
    ///
    /// let mut array = json!([ "A", "B", "C" ]);
    /// *array.get_mut(2).unwrap() = json!("D");
    /// ```
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    /// Returns true if the `Value` is an Object. Returns false otherwise.
    ///
    /// For any Value on which `is_object` returns true, `as_object` and
    /// `as_object_mut` are guaranteed to return the map representation of the
    /// object.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let obj = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// assert!(obj.is_object());
    /// assert!(obj["a"].is_object());
    ///
    /// // array, not an object
    /// assert!(!obj["b"].is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// If the `Value` is an Object, returns the associated Map. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// // The length of `{"nested": true}` is 1 entry.
    /// assert_eq!(v["a"].as_object().unwrap().len(), 1);
    ///
    /// // The array `["an", "array"]` is not an object.
    /// assert_eq!(v["b"].as_object(), None);
    /// ```
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is an Object, returns the associated mutable Map.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "a": { "nested": true } });
    ///
    /// v["a"].as_object_mut().unwrap().clear();
    /// assert_eq!(v, json!({ "a": {} }));
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an Array. Returns false otherwise.
    ///
    /// For any Value on which `is_array` returns true, `as_array` and
    /// `as_array_mut` are guaranteed to return the vector representing the
    /// array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let obj = json!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// assert!(obj["a"].is_array());
    ///
    /// // an object, not an array
    /// assert!(!obj["b"].is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// If the `Value` is an Array, returns the associated vector. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// // The length of `["an", "array"]` is 2 elements.
    /// assert_eq!(v["a"].as_array().unwrap().len(), 2);
    ///
    /// // The object `{"an": "object"}` is not an array.
    /// assert_eq!(v["b"].as_array(), None);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns the associated mutable vector.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "a": ["an", "array"] });
    ///
    /// v["a"].as_array_mut().unwrap().clear();
    /// assert_eq!(v, json!({ "a": [] }));
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(list) => Some(list),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a String. Returns false otherwise.
    ///
    /// For any Value on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": "some string", "b": false });
    ///
    /// assert!(v["a"].is_string());
    ///
    /// // The boolean `false` is not a string.
    /// assert!(!v["b"].is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    /// If the `Value` is a String, returns the associated str. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": "some string", "b": false });
    ///
    /// assert_eq!(v["a"].as_str(), Some("some string"));
    ///
    /// // The boolean `false` is not a string.
    /// assert_eq!(v["b"].as_str(), None);
    ///
    /// // JSON values are printed in JSON representation, so strings are in quotes.
    /// //
    /// //    The value is: "some string"
    /// println!("The value is: {}", v["a"]);
    ///
    /// // Rust strings are printed without quotes.
    /// //
    /// //    The value is: some string
    /// println!("The value is: {}", v["a"].as_str().unwrap());
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 1, "b": "2" });
    ///
    /// assert!(v["a"].is_number());
    ///
    /// // The string `"2"` is a string, not a number.
    /// assert!(!v["b"].is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    /// If the `Value` is a Number, returns the associated [`Number`]. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::{json, Number};
    /// #
    /// let v = json!({ "a": 1, "b": 2.2, "c": -3, "d": "4" });
    ///
    /// assert_eq!(v["a"].as_number(), Some(&Number::from(1u64)));
    /// assert_eq!(v["b"].as_number(), Some(&Number::from_f64(2.2).unwrap()));
    /// assert_eq!(v["c"].as_number(), Some(&Number::from(-3i64)));
    ///
    /// // The string `"4"` is not a number.
    /// assert_eq!(v["d"].as_number(), None);
    /// ```
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(number) => Some(number),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Value on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// ```
    pub fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between zero and `u64::MAX`.
    ///
    /// For any Value on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// ```
    pub fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is a number that can be represented by f64.
    ///
    /// For any Value on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// ```
    pub fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the `Value` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let big = i64::max_value() as u64 + 10;
    /// let v = json!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `Value` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None);
    /// assert_eq!(v["c"].as_u64(), None);
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `Value` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.as_f64(),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Boolean. Returns false otherwise.
    ///
    /// For any Value on which `is_boolean` returns true, `as_bool` is
    /// guaranteed to return the boolean value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": false, "b": "false" });
    ///
    /// assert!(v["a"].is_boolean());
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert!(!v["b"].is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    /// If the `Value` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": false, "b": "false" });
    ///
    /// assert_eq!(v["a"].as_bool(), Some(false));
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert_eq!(v["b"].as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Null. Returns false otherwise.
    ///
    /// For any Value on which `is_null` returns true, `as_null` is guaranteed
    /// to return `Some(())`.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": null, "b": false });
    ///
    /// assert!(v["a"].is_null());
    ///
    /// // The boolean `false` is not null.
    /// assert!(!v["b"].is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    /// If the `Value` is a Null, returns (). Returns None otherwise.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "a": null, "b": false });
    ///
    /// assert_eq!(v["a"].as_null(), Some(()));
    ///
    /// // The boolean `false` is not null.
    /// assert_eq!(v["b"].as_null(), None);
    /// ```
    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    /// Looks up a value by a JSON Pointer.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
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
    /// assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
    /// assert_eq!(data.pointer("/a/b/c"), None);
    /// ```
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                Value::Object(map) => map.get(&token),
                Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => None,
            })
    }

    /// Looks up a value by a JSON Pointer and returns a mutable reference to
    /// that value.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    ///
    /// # Example of Use
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// fn main() {
    ///     let s = r#"{"x": 1.0, "y": 2.0}"#;
    ///     let mut value: Value = serde_json::from_str(s).unwrap();
    ///
    ///     // Check value using read-only pointer
    ///     assert_eq!(value.pointer("/x"), Some(&1.0.into()));
    ///     // Change value with direct assignment
    ///     *value.pointer_mut("/x").unwrap() = 1.5.into();
    ///     // Check that new value was written
    ///     assert_eq!(value.pointer("/x"), Some(&1.5.into()));
    ///     // Or change the value only if it exists
    ///     value.pointer_mut("/x").map(|v| *v = 1.5.into());
    ///
    ///     // "Steal" ownership of a value. Can replace with any valid Value.
    ///     let old_x = value.pointer_mut("/x").map(Value::take).unwrap();
    ///     assert_eq!(old_x, 1.5);
    ///     assert_eq!(value.pointer("/x").unwrap(), &Value::Null);
    /// }
    /// ```
    pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| match target {
                Value::Object(map) => map.get_mut(&token),
                Value::Array(list) => parse_index(&token).and_then(move |x| list.get_mut(x)),
                _ => None,
            })
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let mut v = json!({ "x": "y" });
    /// assert_eq!(v["x"].take(), json!("y"));
    /// assert_eq!(v, json!({ "x": null }));
    /// ```
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    /// Reorders the entries of all `Value::Object` nested within this JSON
    /// value according to `str`'s usual ordering.
    ///
    /// If serde_json's "preserve_order" feature is not enabled, this method
    /// does no work because all JSON maps are always kept in a sorted state.
    ///
    /// If serde_json's "preserve_order" feature is enabled, this method
    /// destroys the original source order or insertion order of the JSON
    /// objects in favor of an alphanumerical order that matches how a BTreeMap
    /// with the same contents would be ordered.
    pub fn sort_all_objects(&mut self) {
        #[cfg(feature = "preserve_order")]
        {
            match self {
                Value::Object(map) => {
                    map.sort_keys();
                    map.values_mut().for_each(Value::sort_all_objects);
                }
                Value::Array(list) => {
                    list.iter_mut().for_each(Value::sort_all_objects);
                }
                _ => {}
            }
        }
    }
}

/// The default value is `Value::Null`.
///
/// This is useful for handling omitted `Value` fields when deserializing.
///
/// # Examples
///
/// ```
/// # use serde::Deserialize;
/// use serde_json::Value;
///
/// #[derive(Deserialize)]
/// struct Settings {
///     level: i32,
///     #[serde(default)]
///     extras: Value,
/// }
///
/// # fn try_main() -> Result<(), serde_json::Error> {
/// let data = r#" { "level": 42 } "#;
/// let s: Settings = serde_json::from_str(data)?;
///
/// assert_eq!(s.level, 42);
/// assert_eq!(s.extras, Value::Null);
/// #
/// #     Ok(())
/// # }
/// #
/// # try_main().unwrap()
/// ```
impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

mod de;
mod from;
mod index;
mod partial_eq;
mod ser;

/// Convert a `T` into `serde_json::Value` which is an enum that can represent
/// any valid JSON data.
///
/// # Example
///
/// ```
/// use serde::Serialize;
/// use serde_json::json;
/// use std::error::Error;
///
/// #[derive(Serialize)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn compare_json_values() -> Result<(), Box<dyn Error>> {
///     let u = User {
///         fingerprint: "0xF9BA143B95FF6D82".to_owned(),
///         location: "Menlo Park, CA".to_owned(),
///     };
///
///     // The type of `expected` is `serde_json::Value`
///     let expected = json!({
///         "fingerprint": "0xF9BA143B95FF6D82",
///         "location": "Menlo Park, CA",
///     });
///
///     let v = serde_json::to_value(u).unwrap();
///     assert_eq!(v, expected);
///
///     Ok(())
/// }
/// #
/// # compare_json_values().unwrap();
/// ```
///
/// # Errors
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
///
/// ```
/// use std::collections::BTreeMap;
///
/// fn main() {
///     // The keys in this map are vectors, not strings.
///     let mut map = BTreeMap::new();
///     map.insert(vec![32, 64], "x86");
///
///     println!("{}", serde_json::to_value(map).unwrap_err());
/// }
/// ```
// Taking by value is more friendly to iterator adapters, option and result
// consumers, etc. See https://github.com/serde-rs/json/pull/149.
pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(Serializer)
}

/// Interpret a `serde_json::Value` as an instance of type `T`.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
/// use serde_json::json;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `serde_json::Value`
///     let j = json!({
///         "fingerprint": "0xF9BA143B95FF6D82",
///         "location": "Menlo Park, CA"
///     });
///
///     let u: User = serde_json::from_value(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}


#[cfg(test)]
mod tests_llm_16_370 {
    use super::*; // Ensure to import the necessary items

use crate::*;
    use crate::Value;

    #[test]
    fn test_default_value() {
        // Asserting that the default value is `Value::Null`
        let default_value = Value::default();
        assert_eq!(default_value, Value::Null);
    }
}

#[cfg(test)]
mod tests_llm_16_747 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_array_with_array() {
        let value = json!([1, 2, 3]);
        let array = value.as_array().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], json!(1));
        assert_eq!(array[1], json!(2));
        assert_eq!(array[2], json!(3));
    }

    #[test]
    fn test_as_array_with_object() {
        let value = json!({ "key": "value" });
        assert_eq!(value.as_array(), None);
    }

    #[test]
    fn test_as_array_with_number() {
        let value = json!(42);
        assert_eq!(value.as_array(), None);
    }

    #[test]
    fn test_as_array_with_string() {
        let value = json!("Hello");
        assert_eq!(value.as_array(), None);
    }

    #[test]
    fn test_as_array_with_bool() {
        let value = json!(true);
        assert_eq!(value.as_array(), None);
    }

    #[test]
    fn test_as_array_with_null() {
        let value = json!(null);
        assert_eq!(value.as_array(), None);
    }

    #[test]
    fn test_as_array_with_empty_array() {
        let value = json!([]);
        let array = value.as_array().unwrap();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_as_array_with_nested_array() {
        let value = json!([json!([1, 2]), json!([3, 4])]);
        let array = value.as_array().unwrap();
        assert_eq!(array.len(), 2);
        assert_eq!(array[0].as_array().unwrap().len(), 2);
        assert_eq!(array[1].as_array().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod tests_llm_16_748 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_array_mut_some() {
        let mut value = json!([1, 2, 3]);
        let array_mut = value.as_array_mut().unwrap();
        array_mut.push(Value::Number(Number::from(4)));
        assert_eq!(value, json!([1, 2, 3, 4]));
    }

    #[test]
    fn test_as_array_mut_none() {
        let mut value = json!({"key": "value"});
        let array_mut = value.as_array_mut();
        assert!(array_mut.is_none());
    }

    #[test]
    fn test_as_array_mut_empty() {
        let mut value = json!([]);
        let array_mut = value.as_array_mut().unwrap();
        array_mut.clear();
        assert_eq!(value, json!([]));
    }

    #[test]
    fn test_as_array_mut_with_values() {
        let mut value = json!([1, 2, 3]);
        {
            let array_mut = value.as_array_mut().unwrap();
            array_mut[1] = Value::Number(Number::from(20));
        }
        assert_eq!(value, json!([1, 20, 3]));
    }

    #[test]
    fn test_as_array_mut_on_non_array() {
        let mut value = json!(true);
        let array_mut = value.as_array_mut();
        assert!(array_mut.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_749 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_bool() {
        let v_true = json!(true);
        let v_false = json!(false);
        let v_string = json!("not a bool");
        let v_null = json!(null);

        assert_eq!(v_true.as_bool(), Some(true));
        assert_eq!(v_false.as_bool(), Some(false));
        assert_eq!(v_string.as_bool(), None);
        assert_eq!(v_null.as_bool(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_750 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_f64() {
        let v1 = json!(256.0);
        let v2 = json!(64);
        let v3 = json!(-64);
        let v4 = json!("string");
        let v5 = json!(true);
        let v6 = json!(null);

        assert_eq!(v1.as_f64(), Some(256.0));
        assert_eq!(v2.as_f64(), Some(64.0));
        assert_eq!(v3.as_f64(), Some(-64.0));
        assert_eq!(v4.as_f64(), None);
        assert_eq!(v5.as_f64(), None);
        assert_eq!(v6.as_f64(), None);
    }

    #[test]
    fn test_as_f64_non_finite() {
        let v_inf = json!(f64::INFINITY);
        let v_nan = json!(f64::NAN);
        
        assert_eq!(v_inf.as_f64(), None);
        assert_eq!(v_nan.as_f64(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_751 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_i64() {
        let v_integer = json!(64);
        let v_large_integer = json!(i64::max_value() as u64 + 10);
        let v_float = json!(256.0);
        let v_negative_integer = json!(-64);

        assert_eq!(v_integer.as_i64(), Some(64));
        assert_eq!(v_large_integer.as_i64(), None);
        assert_eq!(v_float.as_i64(), None);
        assert_eq!(v_negative_integer.as_i64(), Some(-64));
    }

    #[test]
    fn test_as_i64_when_value_is_null() {
        let v_null = json!(null);
        assert_eq!(v_null.as_i64(), None);
    }

    #[test]
    fn test_as_i64_with_non_integer_value() {
        let v_string = json!("not an integer");
        let v_bool = json!(true);
        assert_eq!(v_string.as_i64(), None);
        assert_eq!(v_bool.as_i64(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_752 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_null() {
        let v_null = json!(null);
        let v_false = json!(false);

        assert_eq!(v_null.as_null(), Some(()));
        assert_eq!(v_false.as_null(), None);
    }

    #[test]
    fn test_as_null_with_empty_object() {
        let v_empty = json!({});
        assert_eq!(v_empty.as_null(), None);
    }

    #[test]
    fn test_as_null_with_number() {
        let v_number = json!(42);
        assert_eq!(v_number.as_null(), None);
    }

    #[test]
    fn test_as_null_with_string() {
        let v_string = json!("hello");
        assert_eq!(v_string.as_null(), None);
    }

    #[test]
    fn test_as_null_with_array() {
        let v_array = json!([1, 2, 3]);
        assert_eq!(v_array.as_null(), None);
    }

    #[test]
    fn test_as_null_with_object() {
        let v_object = json!({"key": "value"});
        assert_eq!(v_object.as_null(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_753 {
    use super::*;

use crate::*;
    use crate::{json, Number};

    #[test]
    fn test_as_number() {
        let v = json!({ "a": 1, "b": 2.2, "c": -3, "d": "4" });

        assert_eq!(v["a"].as_number(), Some(&Number::from(1u64)));
        assert_eq!(v["b"].as_number(), Some(&Number::from_f64(2.2).unwrap()));
        assert_eq!(v["c"].as_number(), Some(&Number::from(-3i64)));
        assert_eq!(v["d"].as_number(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_754 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_object() {
        let v = json!({ "a": { "nested": true }, "b": ["an", "array"] });
        
        // Test that as_object returns Some for a valid object
        assert_eq!(v["a"].as_object().unwrap().len(), 1);
        
        // Test that as_object returns None for a non-object
        assert_eq!(v["b"].as_object(), None);
        
        // Test that as_object returns None for a null value
        let v_null = json!(null);
        assert_eq!(v_null.as_object(), None);
        
        // Test that as_object returns None for a boolean value
        let v_bool = json!(true);
        assert_eq!(v_bool.as_object(), None);
        
        // Test that as_object returns None for a number
        let v_number = json!(42);
        assert_eq!(v_number.as_object(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_756 {
    use super::*; // Import the necessary items

use crate::*;
    use crate::json;

    #[test]
    fn test_as_str() {
        let v_string = json!("example string");
        let v_number = json!(42);
        let v_bool = json!(true);
        let v_null = json!(null);

        assert_eq!(v_string.as_str(), Some("example string"));
        assert_eq!(v_number.as_str(), None);
        assert_eq!(v_bool.as_str(), None);
        assert_eq!(v_null.as_str(), None);
    }

    #[test]
    fn test_as_str_nested() {
        let nested_json = json!({
            "message": "hello",
            "details": {
                "info": "world"
            },
            "num": 100
        });

        // Access the outer string
        assert_eq!(nested_json["message"].as_str(), Some("hello"));
        // Accessing nested object string
        assert_eq!(nested_json["details"]["info"].as_str(), Some("world"));
        // Accessing integer
        assert_eq!(nested_json["num"].as_str(), None);
    }

    #[test]
    fn test_as_str_in_array() {
        let array_json = json!(["string1", "string2", 3, false]);

        assert_eq!(array_json[0].as_str(), Some("string1"));
        assert_eq!(array_json[1].as_str(), Some("string2"));
        assert_eq!(array_json[2].as_str(), None);
        assert_eq!(array_json[3].as_str(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_757 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_as_u64() {
        let v1 = json!(64);
        let v2 = json!(-64);
        let v3 = json!(256.0);
        
        assert_eq!(v1.as_u64(), Some(64));
        assert_eq!(v2.as_u64(), None);
        assert_eq!(v3.as_u64(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_758 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_get_from_object() {
        let object = json!({ "A": 65, "B": 66, "C": 67 });
        assert_eq!(*object.get("A").unwrap(), json!(65));
        assert_eq!(object.get("B").unwrap(), &json!(66));
        assert_eq!(object.get("D"), None);
    }

    #[test]
    fn test_get_from_array() {
        let array = json!([ "A", "B", "C" ]);
        assert_eq!(*array.get(2).unwrap(), json!("C"));
        assert_eq!(array.get(0).unwrap(), &json!("A"));
        assert_eq!(array.get(3), None);
    }

    #[test]
    fn test_get_invalid_indexing() {
        let object = json!({ "A": 65, "B": 66, "C": 67 });
        assert_eq!(object.get(0), None);
        assert_eq!(object.get(1), None);
    }

    #[test]
    fn test_get_mut_object() {
        let mut object = json!({ "A": 65, "B": 66, "C": 67 });
        *object.get_mut("A").unwrap() = json!(69);
        assert_eq!(*object.get("A").unwrap(), json!(69));
    }

    #[test]
    fn test_get_mut_array() {
        let mut array = json!([ "A", "B", "C" ]);
        *array.get_mut(2).unwrap() = json!("D");
        assert_eq!(*array.get(2).unwrap(), json!("D"));
    }

    #[test]
    fn test_get_from_nested_object() {
        let object = json!({
            "A": {
                "B": {
                    "C": 67
                }
            }
        });
        assert_eq!(object.get("A").unwrap().get("B").unwrap().get("C"), Some(&json!(67)));
    }
}

#[cfg(test)]
mod tests_llm_16_759 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_get_mut_object() {
        let mut object = json!({ "A": 65, "B": 66, "C": 67 });
        {
            let value = object.get_mut("A").unwrap();
            *value = json!(69);
        }
        assert_eq!(object, json!({ "A": 69, "B": 66, "C": 67 }));
    }

    #[test]
    fn test_get_mut_array() {
        let mut array = json!([ "A", "B", "C" ]);
        {
            let value = array.get_mut(2).unwrap();
            *value = json!("D");
        }
        assert_eq!(array, json!([ "A", "B", "D" ]));
    }

    #[test]
    fn test_get_mut_non_existent_key() {
        let mut object = json!({ "A": 65, "B": 66 });
        let value = object.get_mut("C");
        assert!(value.is_none());
    }

    #[test]
    fn test_get_mut_non_existent_index() {
        let mut array = json!([ "A", "B", "C" ]);
        let value = array.get_mut(3);
        assert!(value.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_760 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_array_with_array() {
        let value = json!(["an", "array"]);
        assert!(value.is_array());
    }

    #[test]
    fn test_is_array_with_object() {
        let value = json!({"key": "value"});
        assert!(!value.is_array());
    }

    #[test]
    fn test_is_array_with_number() {
        let value = json!(42);
        assert!(!value.is_array());
    }

    #[test]
    fn test_is_array_with_string() {
        let value = json!("a string");
        assert!(!value.is_array());
    }

    #[test]
    fn test_is_array_with_boolean() {
        let value = json!(true);
        assert!(!value.is_array());
    }

    #[test]
    fn test_is_array_with_null() {
        let value = json!(null);
        assert!(!value.is_array());
    }
}

#[cfg(test)]
mod tests_llm_16_761 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_boolean() {
        let v_true = json!(true);
        let v_false = json!(false);
        let v_string = json!("false");
        let v_number = json!(1);
        let v_null = json!(null);
        let v_array = json!([]);
        let v_object = json!({});

        assert!(v_true.is_boolean());
        assert!(v_false.is_boolean());
        assert!(!v_string.is_boolean());
        assert!(!v_number.is_boolean());
        assert!(!v_null.is_boolean());
        assert!(!v_array.is_boolean());
        assert!(!v_object.is_boolean());
    }
}

#[cfg(test)]
mod tests_llm_16_762 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_f64() {
        let value_f64 = json!(3.14159);
        let value_i64 = json!(42);
        let value_u64 = json!(100);
        let value_string = json!("not_a_number");
        let value_null = json!(null);
        let value_bool = json!(true);
        
        assert!(value_f64.is_f64());
        assert!(!value_i64.is_f64());
        assert!(!value_u64.is_f64());
        assert!(!value_string.is_f64());
        assert!(!value_null.is_f64());
        assert!(!value_bool.is_f64());
    }
}

#[cfg(test)]
mod tests_llm_16_763 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_i64() {
        let v1 = json!(64);
        assert!(v1.is_i64());

        let v2 = json!(i64::MAX);
        assert!(v2.is_i64());

        let v3 = json!(i64::MAX as u64 + 1);
        assert!(!v3.is_i64());

        let v4 = json!(256.0);
        assert!(!v4.is_i64());

        let v5 = json!(-64);
        assert!(v5.is_i64());

        let v6 = json!(u64::MAX);
        assert!(!v6.is_i64());

        let v7 = json!(null);
        assert!(!v7.is_i64());

        let v8 = json!("string");
        assert!(!v8.is_i64());

        let v9 = json!({});
        assert!(!v9.is_i64());
    }
}

#[cfg(test)]
mod tests_llm_16_764 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_null() {
        let v_null = json!(null);
        let v_true = json!(true);
        let v_false = json!(false);
        let v_number = json!(1);
        let v_string = json!("test");
        let v_array = json!([1, 2, 3]);
        let v_object = json!({"key": "value"});

        assert!(v_null.is_null());
        assert!(!v_true.is_null());
        assert!(!v_false.is_null());
        assert!(!v_number.is_null());
        assert!(!v_string.is_null());
        assert!(!v_array.is_null());
        assert!(!v_object.is_null());
    }
}

#[cfg(test)]
mod tests_llm_16_766 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_object() {
        let obj = json!({ "a": { "nested": true }, "b": ["an", "array"] });
        assert!(obj.is_object());
        assert!(obj["a"].is_object());
        assert!(!obj["b"].is_object());

        let array = json!(["an", "array"]);
        assert!(!array.is_object());

        let null_value = json!(null);
        assert!(!null_value.is_object());

        let boolean_value = json!(true);
        assert!(!boolean_value.is_object());

        let number_value = json!(42);
        assert!(!number_value.is_object());
    }
}

#[cfg(test)]
mod tests_llm_16_768 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_is_u64() {
        let v1 = json!(64);
        let v2 = json!(-64);
        let v3 = json!(256.0);
        let v4 = json!(u64::MAX);
        let v5 = json!(0);

        assert!(v1.is_u64());
        assert!(!v2.is_u64());
        assert!(!v3.is_u64());
        assert!(v4.is_u64());
        assert!(v5.is_u64());
    }
}

#[cfg(test)]
mod tests_llm_16_769 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_pointer() {
        let data = json!({
            "x": {
                "y": ["z", "zz"]
            }
        });

        assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
        assert_eq!(data.pointer("/a/b/c"), None);
        assert_eq!(data.pointer(""), Some(&data));
        assert_eq!(data.pointer("/x/y/0").unwrap(), &json!("z"));
        assert_eq!(data.pointer("/x/y"), None);
        assert_eq!(data.pointer("/x/y/3"), None);
        assert_eq!(data.pointer("/x/y/1/0"), None);
    }
}

#[cfg(test)]
mod tests_llm_16_771 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    #[cfg(feature = "preserve_order")]
    fn test_sort_single_object() {
        let mut json_value = json!({
            "c": 3,
            "a": 1,
            "b": 2
        });
        json_value.sort_all_objects();
        let expected = json!({
            "a": 1,
            "b": 2,
            "c": 3
        });
        assert_eq!(json_value, expected);
    }

    #[test]
    #[cfg(feature = "preserve_order")]
    fn test_sort_nested_objects() {
        let mut json_value = json!({
            "b": {
                "a": 1,
                "c": 3
            },
            "a": 2
        });
        json_value.sort_all_objects();
        let expected = json!({
            "a": 2,
            "b": {
                "a": 1,
                "c": 3
            }
        });
        assert_eq!(json_value, expected);
    }

    #[test]
    #[cfg(feature = "preserve_order")]
    fn test_sort_array_of_objects() {
        let mut json_value = json!([
            { "b": 2, "a": 1 },
            { "c": 3, "a": 1 }
        ]);
        json_value.sort_all_objects();
        let expected = json!([
            { "a": 1, "b": 2 },
            { "a": 1, "c": 3 }
        ]);
        assert_eq!(json_value, expected);
    }

    #[test]
    #[cfg(feature = "preserve_order")]
    fn test_sort_empty_object() {
        let mut json_value = json!({});
        json_value.sort_all_objects();
        let expected = json!({});
        assert_eq!(json_value, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_772 {
    use super::*;

use crate::*;
    use crate::json;

    #[test]
    fn test_take_method() {
        let mut v = json!({ "x": "y" });
        assert_eq!(v["x"].take(), json!("y"));
        assert_eq!(v, json!({ "x": null }));
    }
}

#[cfg(test)]
mod tests_llm_16_938 {
    use crate::value::parse_index;

    #[test]
    fn test_parse_index_valid() {
        assert_eq!(parse_index("0"), Some(0));
        assert_eq!(parse_index("1"), Some(1));
        assert_eq!(parse_index("123"), Some(123));
    }

    #[test]
    fn test_parse_index_invalid() {
        assert_eq!(parse_index("+1"), None);
        assert_eq!(parse_index("01"), None);
        assert_eq!(parse_index("0abc"), None);
        assert_eq!(parse_index("abc"), None);
        assert_eq!(parse_index(""), None);
    }

    #[test]
    fn test_parse_index_edge_cases() {
        assert_eq!(parse_index("0"), Some(0));
        assert_eq!(parse_index(""), None);
    }
}
