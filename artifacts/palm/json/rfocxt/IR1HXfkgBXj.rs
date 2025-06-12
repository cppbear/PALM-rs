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
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}
pub trait Read<'de>: private::Sealed {
    const should_early_return_if_failed: bool;
    fn next(&mut self) -> Result<Option<u8>>;
    fn peek(&mut self) -> Result<Option<u8>>;
    fn discard(&mut self);
    fn position(&self) -> Position;
    fn peek_position(&self) -> Position;
    fn byte_offset(&self) -> usize;
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>>;
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;
    fn ignore_str(&mut self) -> Result<()>;
    fn decode_hex_escape(&mut self) -> Result<u16>;
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self);
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;
    fn set_failed(&mut self, failed: &mut bool);
}
pub(crate) trait FloatErrors {
    fn error_scale() -> u32;
    fn error_halfscale() -> u32;
    fn error_is_accurate<F: Float>(count: u32, fp: &ExtendedFloat) -> bool;
}
pub trait Float: Number {
    type Unsigned: Integer;
    const ZERO: Self;
    const MAX_DIGITS: usize;
    const EXPONENT_MASK: Self::Unsigned;
    const HIDDEN_BIT_MASK: Self::Unsigned;
    const MANTISSA_MASK: Self::Unsigned;
    const INFINITY_BITS: Self::Unsigned;
    const MANTISSA_SIZE: i32;
    const EXPONENT_BIAS: i32;
    const DENORMAL_EXPONENT: i32;
    const MAX_EXPONENT: i32;
    const DEFAULT_SHIFT: i32;
    const CARRY_MASK: u64;
    fn exponent_limit() -> (i32, i32);
    fn mantissa_limit() -> i32;
    fn pow10(self, n: i32) -> Self;
    fn from_bits(u: Self::Unsigned) -> Self;
    fn to_bits(self) -> Self::Unsigned;
    fn is_sign_positive(self) -> bool;
    #[inline]
    fn is_denormal(self) -> bool;
    #[inline]
    fn is_special(self) -> bool;
    #[inline]
    fn is_inf(self) -> bool;
    #[inline]
    fn exponent(self) -> i32;
    #[inline]
    fn mantissa(self) -> Self::Unsigned {
        let bits = self.to_bits();
        let s = bits & Self::MANTISSA_MASK;
        if !self.is_denormal() { s + Self::HIDDEN_BIT_MASK } else { s }
    }
    #[inline]
    fn next_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && ! self.is_inf());
        Self::from_bits(self.to_bits() + Self::Unsigned::as_cast(1u32))
    }
    #[inline]
    fn round_positive_even(self) -> Self {
        if self.mantissa() & Self::Unsigned::as_cast(1u32)
            == Self::Unsigned::as_cast(1u32)
        {
            self.next_positive()
        } else {
            self
        }
    }
}
trait Hi64<T>: AsRef<[T]> {
    fn hi64_1(&self) -> (u64, bool);
    fn hi64_2(&self) -> (u64, bool);
    fn hi64_3(&self) -> (u64, bool);
    #[inline]
    fn hi64(&self) -> (u64, bool);
}
pub trait Index: private::Sealed {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}
pub struct Map<K, V> {
    map: MapImpl<K, V>,
}
pub struct Deserializer<R> {
    read: R,
    scratch: Vec<u8>,
    remaining_depth: u8,
    #[cfg(feature = "float_roundtrip")]
    single_precision: bool,
    #[cfg(feature = "unbounded_depth")]
    disable_recursion_limit: bool,
}
pub struct Serializer;
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
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
impl<'de> de::IntoDeserializer<'de, Error> for &'de Map<String, Value> {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
