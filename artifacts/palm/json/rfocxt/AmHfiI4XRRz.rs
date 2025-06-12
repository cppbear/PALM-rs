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
pub struct OccupiedEntry<'a> {
    occupied: OccupiedEntryImpl<'a>,
}
pub struct VacantEntry<'a> {
    vacant: VacantEntryImpl<'a>,
}
pub enum Entry<'a> {
    /// A vacant Entry.
    Vacant(VacantEntry<'a>),
    /// An occupied Entry.
    Occupied(OccupiedEntry<'a>),
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
impl<'a> Entry<'a> {
    pub fn key(&self) -> &String {}
    pub fn or_insert(self, default: Value) -> &'a mut Value {}
    pub fn or_insert_with<F>(self, default: F) -> &'a mut Value
    where
        F: FnOnce() -> Value,
    {
        match self {
            Entry::Vacant(entry) => entry.insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Value),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}
impl<'a> OccupiedEntry<'a> {
    #[inline]
    pub fn key(&self) -> &String {}
    #[inline]
    pub fn get(&self) -> &Value {}
    #[inline]
    pub fn get_mut(&mut self) -> &mut Value {}
    #[inline]
    pub fn into_mut(self) -> &'a mut Value {
        self.occupied.into_mut()
    }
    #[inline]
    pub fn insert(&mut self, value: Value) -> Value {}
    #[inline]
    pub fn remove(self) -> Value {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn swap_remove(self) -> Value {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn shift_remove(self) -> Value {}
    #[inline]
    pub fn remove_entry(self) -> (String, Value) {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn swap_remove_entry(self) -> (String, Value) {}
    #[cfg(feature = "preserve_order")]
    #[cfg_attr(docsrs, doc(cfg(feature = "preserve_order")))]
    #[inline]
    pub fn shift_remove_entry(self) -> (String, Value) {}
}
impl<'a> VacantEntry<'a> {
    #[inline]
    pub fn key(&self) -> &String {}
    #[inline]
    pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }
}
