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
#[derive(Copy, Clone)]
struct Pos {
    index: Size,
    hash: HashValue,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: Repr<Custom>,
}
#[derive(Debug, Clone)]
struct Bucket<T> {
    hash: HashValue,
    key: HeaderName,
    value: T,
    links: Option<Links>,
}
pub struct MaxSizeReached {
    _priv: (),
}
#[derive(Debug, Clone)]
struct ExtraValue<T> {
    value: T,
    prev: Link,
    next: Link,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HashValue(u16);
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
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {}
    pub fn keys(&self) -> Keys<'_, T> {}
    pub fn values(&self) -> Values<'_, T> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, T> {}
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
    {
        self.try_reserve_one()?;
        Ok(
            insert_phase_one!(
                self, key, probe, pos, hash, danger, { let _ = danger; let index = self
                .entries.len(); self.try_insert_entry(hash, key.into(), value) ?; self
                .indices[probe] = Pos::new(index, hash); None }, Some(self
                .insert_occupied(pos, value)), { self.try_insert_phase_two(key.into(),
                value, hash, probe, danger) ?; None }
            ),
        )
    }
    #[inline]
    fn insert_occupied(&mut self, index: usize, value: T) -> T {
        if let Some(links) = self.entries[index].links {
            self.remove_all_extra_values(links.next);
        }
        let entry = &mut self.entries[index];
        mem::replace(&mut entry.value, value)
    }
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
    ) -> Result<usize, MaxSizeReached> {
        let index = self.entries.len();
        self.try_insert_entry(hash, key, value)?;
        let num_displaced = do_insert_phase_two(
            &mut self.indices,
            probe,
            Pos::new(index, hash),
        );
        if danger || num_displaced >= DISPLACEMENT_THRESHOLD {
            self.danger.set_yellow();
        }
        Ok(index)
    }
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
    ) -> Result<(), MaxSizeReached> {
        if self.entries.len() >= MAX_SIZE {
            return Err(MaxSizeReached::new());
        }
        self.entries
            .push(Bucket {
                hash,
                key,
                value,
                links: None,
            });
        Ok(())
    }
    fn rebuild(&mut self) {}
    fn reinsert_entry_in_order(&mut self, pos: Pos) {}
    fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
        let len = self.entries.len();
        if self.danger.is_yellow() {
            let load_factor = self.entries.len() as f32 / self.indices.len() as f32;
            if load_factor >= LOAD_FACTOR_THRESHOLD {
                self.danger.set_green();
                let new_cap = self.indices.len() * 2;
                self.try_grow(new_cap)?;
            } else {
                self.danger.set_red();
                for index in self.indices.iter_mut() {
                    *index = Pos::none();
                }
                self.rebuild();
            }
        } else if len == self.capacity() {
            if len == 0 {
                let new_raw_cap = 8;
                self.mask = 8 - 1;
                self.indices = vec![Pos::none(); new_raw_cap].into_boxed_slice();
                self.entries = Vec::with_capacity(usable_capacity(new_raw_cap));
            } else {
                let raw_cap = self.indices.len();
                self.try_grow(raw_cap << 1)?;
            }
        }
        Ok(())
    }
    #[inline]
    fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {}
    #[inline]
    fn raw_links(&mut self) -> RawLinks<T> {}
}
impl Pos {
    #[inline]
    fn new(index: usize, hash: HashValue) -> Self {
        debug_assert!(index < MAX_SIZE);
        Pos { index: index as Size, hash }
    }
    #[inline]
    fn none() -> Self {
        Pos {
            index: !0,
            hash: HashValue(0),
        }
    }
    #[inline]
    fn is_some(&self) -> bool {}
    #[inline]
    fn is_none(&self) -> bool {}
    #[inline]
    fn resolve(&self) -> Option<(usize, HashValue)> {
        if self.is_some() { Some((self.index as usize, self.hash)) } else { None }
    }
}
impl Danger {
    fn is_red(&self) -> bool {
        matches!(* self, Danger::Red(_))
    }
    fn set_red(&mut self) {}
    fn is_yellow(&self) -> bool {}
    fn set_yellow(&mut self) {}
    fn set_green(&mut self) {}
}
#[inline]
fn desired_pos(mask: Size, hash: HashValue) -> usize {
    (hash.0 & mask) as usize
}
fn hash_elem_using<K>(danger: &Danger, k: &K) -> HashValue
where
    K: Hash + ?Sized,
{
    use fnv::FnvHasher;
    const MASK: u64 = (MAX_SIZE as u64) - 1;
    let hash = match *danger {
        Danger::Red(ref hasher) => {
            let mut h = hasher.build_hasher();
            k.hash(&mut h);
            h.finish()
        }
        _ => {
            let mut h = FnvHasher::default();
            k.hash(&mut h);
            h.finish()
        }
    };
    HashValue((hash & MASK) as u16)
}
#[inline]
fn probe_distance(mask: Size, hash: HashValue, current: usize) -> usize {
    current.wrapping_sub(desired_pos(mask, hash)) & mask as usize
}
