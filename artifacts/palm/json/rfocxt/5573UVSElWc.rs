#[cfg(not(feature = "preserve_order"))]
type MapImpl<K, V> = BTreeMap<K, V>;
#[cfg(feature = "preserve_order")]
type MapImpl<K, V> = IndexMap<K, V>;
#[cfg(not(feature = "preserve_order"))]
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type VacantEntryImpl<'a> = indexmap::map::VacantEntry<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type OccupiedEntryImpl<'a> = indexmap::map::OccupiedEntry<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IterImpl<'a> = btree_map::Iter<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type IterImpl<'a> = indexmap::map::Iter<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IterMutImpl<'a> = btree_map::IterMut<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type IterMutImpl<'a> = indexmap::map::IterMut<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IntoIterImpl = btree_map::IntoIter<String, Value>;
#[cfg(feature = "preserve_order")]
type IntoIterImpl = indexmap::map::IntoIter<String, Value>;
#[cfg(not(feature = "preserve_order"))]
type KeysImpl<'a> = btree_map::Keys<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type KeysImpl<'a> = indexmap::map::Keys<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type ValuesImpl<'a> = btree_map::Values<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesImpl<'a> = indexmap::map::Values<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, String, Value>;
#[cfg(feature = "preserve_order")]
type ValuesMutImpl<'a> = indexmap::map::ValuesMut<'a, String, Value>;
#[cfg(not(feature = "preserve_order"))]
type IntoValuesImpl = btree_map::IntoValues<String, Value>;
#[cfg(feature = "preserve_order")]
type IntoValuesImpl = indexmap::map::IntoValues<String, Value>;
use crate::error::Error;
use crate::value::Value;
use alloc::string::String;
#[cfg(feature = "preserve_order")]
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::iter::FusedIterator;
#[cfg(feature = "preserve_order")]
use core::mem;
use core::ops;
use serde::de;
#[cfg(not(feature = "preserve_order"))]
use alloc::collections::{btree_map, BTreeMap};
#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
macro_rules! delegate_iterator {
    (($name:ident $($generics:tt)*) => $item:ty) => {
        impl $($generics)* Iterator for $name $($generics)* { type Item = $item;
        #[inline] fn next(& mut self) -> Option < Self::Item > { self.iter.next() }
        #[inline] fn size_hint(& self) -> (usize, Option < usize >) { self.iter
        .size_hint() } } impl $($generics)* DoubleEndedIterator for $name $($generics)* {
        #[inline] fn next_back(& mut self) -> Option < Self::Item > { self.iter
        .next_back() } } impl $($generics)* ExactSizeIterator for $name $($generics)* {
        #[inline] fn len(& self) -> usize { self.iter.len() } } impl $($generics)*
        FusedIterator for $name $($generics)* {}
    };
}
delegate_iterator!((Iter <'a >) => (&'a String, &'a Value));
delegate_iterator!((IterMut <'a >) => (&'a String, &'a mut Value));
delegate_iterator!((IntoIter) => (String, Value));
delegate_iterator!((Keys <'a >) => &'a String);
delegate_iterator!((Values <'a >) => &'a Value);
delegate_iterator!((ValuesMut <'a >) => &'a mut Value);
delegate_iterator!((IntoValues) => Value);
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
    pub fn clear(&mut self) {
        self.map.clear();
    }
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
    pub fn len(&self) -> usize {}
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
