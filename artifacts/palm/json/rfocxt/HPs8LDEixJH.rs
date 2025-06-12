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
pub trait Number: AsCast + ops::Add<Output = Self> {}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
pub struct Map<K, V> {
    map: MapImpl<K, V>,
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
impl Value {
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {}
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {}
    pub fn is_object(&self) -> bool {}
    pub fn as_object(&self) -> Option<&Map<String, Value>> {}
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {}
    pub fn is_array(&self) -> bool {}
    pub fn as_array(&self) -> Option<&Vec<Value>> {}
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {}
    pub fn is_string(&self) -> bool {}
    pub fn as_str(&self) -> Option<&str> {}
    pub fn is_number(&self) -> bool {}
    pub fn as_number(&self) -> Option<&Number> {}
    pub fn is_i64(&self) -> bool {}
    pub fn is_u64(&self) -> bool {}
    pub fn is_f64(&self) -> bool {}
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_i64(),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {}
    pub fn as_f64(&self) -> Option<f64> {}
    pub fn is_boolean(&self) -> bool {}
    pub fn as_bool(&self) -> Option<bool> {}
    pub fn is_null(&self) -> bool {}
    pub fn as_null(&self) -> Option<()> {}
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {}
    pub fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value> {}
    pub fn take(&mut self) -> Value {}
    pub fn sort_all_objects(&mut self) {}
}
impl Number {
    pub fn is_i64(&self) -> bool {}
    pub fn is_u64(&self) -> bool {}
    pub fn is_f64(&self) -> bool {}
    pub fn as_i64(&self) -> Option<i64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => if n <= i64::MAX as u64 { Some(n as i64) } else { None }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")] self.n.parse().ok()
    }
    pub fn as_u64(&self) -> Option<u64> {}
    pub fn as_f64(&self) -> Option<f64> {}
    pub fn from_f64(f: f64) -> Option<Number> {}
    pub fn as_i128(&self) -> Option<i128> {}
    pub fn as_u128(&self) -> Option<u128> {}
    pub fn from_i128(i: i128) -> Option<Number> {}
    pub fn from_u128(i: u128) -> Option<Number> {}
    #[cfg(feature = "arbitrary_precision")]
    #[cfg_attr(docsrs, doc(cfg(feature = "arbitrary_precision")))]
    pub fn as_str(&self) -> &str {}
    pub(crate) fn as_f32(&self) -> Option<f32> {}
    pub(crate) fn from_f32(f: f32) -> Option<Number> {}
    #[cfg(feature = "arbitrary_precision")]
    #[inline]
    pub fn from_string_unchecked(n: String) -> Self {
        Number { n }
    }
}
