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
pub struct IterMut<'a, T> {
    map: *mut HeaderMap<T>,
    entry: usize,
    cursor: Option<Cursor>,
    lt: PhantomData<&'a mut HeaderMap<T>>,
}
#[derive(Debug, Copy, Clone)]
struct Links {
    next: usize,
    tail: usize,
}
#[derive(Debug, Clone)]
struct Bucket<T> {
    hash: HashValue,
    key: HeaderName,
    value: T,
    links: Option<Links>,
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: Repr<Custom>,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cursor {
    Head,
    Values(usize),
}
impl<'a, T> IterMut<'a, T> {
    fn next_unsafe(&mut self) -> Option<(&'a HeaderName, *mut T)> {
        use self::Cursor::*;
        if self.cursor.is_none() {
            if (self.entry + 1) >= unsafe { &*self.map }.entries.len() {
                return None;
            }
            self.entry += 1;
            self.cursor = Some(Cursor::Head);
        }
        let entry = unsafe { &mut (*self.map).entries[self.entry] };
        match self.cursor.unwrap() {
            Head => {
                self.cursor = entry.links.map(|l| Values(l.next));
                Some((&entry.key, &mut entry.value as *mut _))
            }
            Values(idx) => {
                let extra = unsafe { &mut (*self.map).extra_values[idx] };
                match extra.next {
                    Link::Entry(_) => self.cursor = None,
                    Link::Extra(i) => self.cursor = Some(Values(i)),
                }
                Some((&entry.key, &mut extra.value as *mut _))
            }
        }
    }
}
