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
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
struct MapDeserializer {
    iter: <Map<String, Value> as IntoIterator>::IntoIter,
    value: Option<Value>,
}
pub struct Error;
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub struct IntoIter {
    iter: IntoIterImpl,
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
impl<'de> serde::Deserializer<'de> for Map<String, Value> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.len();
        let mut deserializer = MapDeserializer::new(self);
        let map = tri!(visitor.visit_map(& mut deserializer));
        let remaining = deserializer.iter.len();
        if remaining == 0 {
            Ok(map)
        } else {
            Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
        }
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut iter = self.into_iter();
        let (variant, value) = match iter.next() {
            Some(v) => v,
            None => {
                return Err(
                    serde::de::Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ),
                );
            }
        };
        if iter.next().is_some() {
            return Err(
                serde::de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                ),
            );
        }
        visitor
            .visit_enum(EnumDeserializer {
                variant,
                value: Some(value),
            })
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
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
impl MapDeserializer {
    fn new(map: Map<String, Value>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}
