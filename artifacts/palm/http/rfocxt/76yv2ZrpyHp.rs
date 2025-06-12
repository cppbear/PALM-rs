type Size = u16;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{BuildHasher, Hash, Hasher};
use std::iter::{FromIterator, FusedIterator};
use std::marker::PhantomData;
use std::{fmt, mem, ops, ptr, vec};
use crate::Error;
use super::name::{HdrName, HeaderName, InvalidHeaderName};
use super::HeaderValue;
pub use self::as_header_name::AsHeaderName;
pub use self::into_header_name::IntoHeaderName;
const MAX_SIZE: usize = 1 << 15;
const DISPLACEMENT_THRESHOLD: usize = 128;
const FORWARD_SHIFT_THRESHOLD: usize = 512;
const LOAD_FACTOR_THRESHOLD: f32 = 0.2;
macro_rules! probe_loop {
    ($label:tt : $probe_var:ident < $len:expr, $body:expr) => {
        debug_assert!($len > 0); $label : loop { if $probe_var < $len { $body $probe_var
        += 1; } else { $probe_var = 0; } }
    };
    ($probe_var:ident < $len:expr, $body:expr) => {
        debug_assert!($len > 0); loop { if $probe_var < $len { $body $probe_var += 1; }
        else { $probe_var = 0; } }
    };
}
macro_rules! insert_phase_one {
    (
        $map:ident, $key:expr, $probe:ident, $pos:ident, $hash:ident, $danger:ident,
        $vacant:expr, $occupied:expr, $robinhood:expr
    ) => {
        { let $hash = hash_elem_using(&$map .danger, &$key); let mut $probe =
        desired_pos($map .mask, $hash); let mut dist = 0; let ret; probe_loop!('probe :
        $probe < $map .indices.len(), { if let Some(($pos, entry_hash)) = $map
        .indices[$probe].resolve() { let their_dist = probe_distance($map .mask,
        entry_hash, $probe); if their_dist < dist { let $danger = dist >=
        FORWARD_SHIFT_THRESHOLD && !$map .danger.is_red(); ret = $robinhood; break
        'probe; } else if entry_hash == $hash && $map .entries[$pos].key == $key { ret =
        $occupied; break 'probe; } } else { let $danger = dist >= FORWARD_SHIFT_THRESHOLD
        && !$map .danger.is_red(); ret = $vacant; break 'probe; } dist += 1; }); ret }
    };
}
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
#[derive(Debug, Clone)]
struct ExtraValue<T> {
    value: T,
    prev: Link,
    next: Link,
}
#[derive(Debug)]
pub struct IterMut<'a, T> {
    map: *mut HeaderMap<T>,
    entry: usize,
    cursor: Option<Cursor>,
    lt: PhantomData<&'a mut HeaderMap<T>>,
}
#[derive(Debug)]
pub struct ValuesMut<'a, T> {
    inner: IterMut<'a, T>,
}
#[derive(Debug, Clone)]
struct Bucket<T> {
    hash: HashValue,
    key: HeaderName,
    value: T,
    links: Option<Links>,
}
#[derive(Copy, Clone)]
struct Pos {
    index: Size,
    hash: HashValue,
}
#[derive(Clone)]
enum Danger {
    Green,
    Yellow,
    Red(RandomState),
}
impl<T> HeaderMap<T> {
    pub fn with_capacity(capacity: usize) -> HeaderMap<T> {}
    pub fn try_with_capacity(capacity: usize) -> Result<HeaderMap<T>, MaxSizeReached> {}
    pub fn len(&self) -> usize {}
    pub fn keys_len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn clear(&mut self) {}
    pub fn capacity(&self) -> usize {}
    pub fn reserve(&mut self, additional: usize) {}
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {}
    pub fn get<K>(&self, key: K) -> Option<&T>
    where
        K: AsHeaderName,
    {}
    fn get2<K>(&self, key: &K) -> Option<&T>
    where
        K: AsHeaderName,
    {}
    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut T>
    where
        K: AsHeaderName,
    {}
    pub fn get_all<K>(&self, key: K) -> GetAll<'_, T>
    where
        K: AsHeaderName,
    {}
    pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsHeaderName,
    {}
    pub fn iter(&self) -> Iter<'_, T> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            map: self as *mut _,
            entry: 0,
            cursor: self.entries.first().map(|_| Cursor::Head),
            lt: PhantomData,
        }
    }
    pub fn keys(&self) -> Keys<'_, T> {}
    pub fn values(&self) -> Values<'_, T> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, T> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }
    pub fn drain(&mut self) -> Drain<'_, T> {}
    fn value_iter(&self, idx: Option<usize>) -> ValueIter<'_, T> {}
    fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<'_, T> {}
    pub fn entry<K>(&mut self, key: K) -> Entry<'_, T>
    where
        K: IntoHeaderName,
    {}
    pub fn try_entry<K>(&mut self, key: K) -> Result<Entry<'_, T>, InvalidHeaderName>
    where
        K: AsHeaderName,
    {}
    fn try_entry2<K>(&mut self, key: K) -> Result<Entry<'_, T>, MaxSizeReached>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {}
    pub fn insert<K>(&mut self, key: K, val: T) -> Option<T>
    where
        K: IntoHeaderName,
    {}
    pub fn try_insert<K>(&mut self, key: K, val: T) -> Result<Option<T>, MaxSizeReached>
    where
        K: IntoHeaderName,
    {}
    #[inline]
    fn try_insert2<K>(&mut self, key: K, value: T) -> Result<Option<T>, MaxSizeReached>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {}
    #[inline]
    fn insert_occupied(&mut self, index: usize, value: T) -> T {}
    fn insert_occupied_mult(&mut self, index: usize, value: T) -> ValueDrain<'_, T> {}
    pub fn append<K>(&mut self, key: K, value: T) -> bool
    where
        K: IntoHeaderName,
    {}
    pub fn try_append<K>(&mut self, key: K, value: T) -> Result<bool, MaxSizeReached>
    where
        K: IntoHeaderName,
    {}
    #[inline]
    fn try_append2<K>(&mut self, key: K, value: T) -> Result<bool, MaxSizeReached>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {}
    #[inline]
    fn find<K>(&self, key: &K) -> Option<(usize, usize)>
    where
        K: Hash + Into<HeaderName> + ?Sized,
        HeaderName: PartialEq<K>,
    {}
    #[inline]
    fn try_insert_phase_two(
        &mut self,
        key: HeaderName,
        value: T,
        hash: HashValue,
        probe: usize,
        danger: bool,
    ) -> Result<usize, MaxSizeReached> {}
    pub fn remove<K>(&mut self, key: K) -> Option<T>
    where
        K: AsHeaderName,
    {}
    #[inline]
    fn remove_found(&mut self, probe: usize, found: usize) -> Bucket<T> {}
    #[inline]
    fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<T> {}
    fn remove_all_extra_values(&mut self, mut head: usize) {}
    #[inline]
    fn try_insert_entry(
        &mut self,
        hash: HashValue,
        key: HeaderName,
        value: T,
    ) -> Result<(), MaxSizeReached> {}
    fn rebuild(&mut self) {}
    fn reinsert_entry_in_order(&mut self, pos: Pos) {}
    fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {}
    #[inline]
    fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {}
    #[inline]
    fn raw_links(&mut self) -> RawLinks<T> {}
}
