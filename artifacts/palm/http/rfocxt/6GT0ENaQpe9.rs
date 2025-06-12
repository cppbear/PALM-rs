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
#[derive(Debug)]
pub struct OccupiedEntry<'a, T> {
    map: &'a mut HeaderMap<T>,
    probe: usize,
    index: usize,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: Repr<Custom>,
}
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
impl<'a, T> OccupiedEntry<'a, T> {
    pub fn key(&self) -> &HeaderName {}
    pub fn get(&self) -> &T {}
    pub fn get_mut(&mut self) -> &mut T {}
    pub fn into_mut(self) -> &'a mut T {}
    pub fn insert(&mut self, value: T) -> T {}
    pub fn insert_mult(&mut self, value: T) -> ValueDrain<'_, T> {}
    pub fn append(&mut self, value: T) {}
    pub fn remove(self) -> T {
        self.remove_entry().1
    }
    pub fn remove_entry(self) -> (HeaderName, T) {
        if let Some(links) = self.map.entries[self.index].links {
            self.map.remove_all_extra_values(links.next);
        }
        let entry = self.map.remove_found(self.probe, self.index);
        (entry.key, entry.value)
    }
    pub fn remove_entry_mult(self) -> (HeaderName, ValueDrain<'a, T>) {}
    pub fn iter(&self) -> ValueIter<'_, T> {}
    pub fn iter_mut(&mut self) -> ValueIterMut<'_, T> {}
}
