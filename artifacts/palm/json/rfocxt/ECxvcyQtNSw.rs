use crate::error::{Error, ErrorCode, Result};
use crate::map::Map;
use crate::value::{to_value, Value};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use core::result;
use serde::ser::{Impossible, Serialize};
pub trait Number: AsCast + ops::Add<Output = Self> {}
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
pub struct Error;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}
pub struct Iter<'a> {
    iter: IterImpl<'a>,
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
impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(v) => v.serialize(serializer),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = tri!(serializer.serialize_map(Some(m.len())));
                for (k, v) in m {
                    tri!(map.serialize_entry(k, v));
                }
                map.end()
            }
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Value::Object(_) => unreachable!(),
        }
    }
}
impl Map<String, Value> {
    #[inline]
    pub fn new() -> Self {
        Map { map: MapImpl::new() }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            #[cfg(not(feature = "preserve_order"))]
            map: {
                let _ = capacity;
                BTreeMap::new()
            },
            #[cfg(feature = "preserve_order")]
            map: IndexMap::with_capacity(capacity),
        }
    }
    #[inline]
    pub fn clear(&mut self) {}
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[inline]
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn shift_insert(&mut self, index: usize, k: String, v: Value) -> Option<Value> {}
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[inline]
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn shift_remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {}
    #[inline]
    pub fn append(&mut self, other: &mut Self) {}
    pub fn entry<S>(&mut self, key: S) -> Entry
    where
        S: Into<String>,
    {}
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn iter(&self) -> Iter {}
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {}
    #[inline]
    pub fn keys(&self) -> Keys {}
    #[inline]
    pub fn values(&self) -> Values {}
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut {}
    #[inline]
    pub fn into_values(self) -> IntoValues {}
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&String, &mut Value) -> bool,
    {}
    #[inline]
    pub fn sort_keys(&mut self) {}
}
