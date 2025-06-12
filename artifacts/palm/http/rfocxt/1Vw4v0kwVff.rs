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
#[derive(Debug, Clone)]
struct ExtraValue<T> {
    value: T,
    prev: Link,
    next: Link,
}
#[derive(Debug)]
struct RawLinks<T>(*mut [Bucket<T>]);
fn drain_all_extra_values<T>(
    raw_links: RawLinks<T>,
    extra_values: &mut Vec<ExtraValue<T>>,
    mut head: usize,
) -> Vec<T> {
    let mut vec = Vec::new();
    loop {
        let extra = remove_extra_value(raw_links, extra_values, head);
        vec.push(extra.value);
        if let Link::Extra(idx) = extra.next {
            head = idx;
        } else {
            break;
        }
    }
    vec
}
#[inline]
fn remove_extra_value<T>(
    mut raw_links: RawLinks<T>,
    extra_values: &mut Vec<ExtraValue<T>>,
    idx: usize,
) -> ExtraValue<T> {
    let prev;
    let next;
    {
        debug_assert!(extra_values.len() > idx);
        let extra = &extra_values[idx];
        prev = extra.prev;
        next = extra.next;
    }
    match (prev, next) {
        (Link::Entry(prev), Link::Entry(next)) => {
            debug_assert_eq!(prev, next);
            raw_links[prev] = None;
        }
        (Link::Entry(prev), Link::Extra(next)) => {
            debug_assert!(raw_links[prev].is_some());
            raw_links[prev].as_mut().unwrap().next = next;
            debug_assert!(extra_values.len() > next);
            extra_values[next].prev = Link::Entry(prev);
        }
        (Link::Extra(prev), Link::Entry(next)) => {
            debug_assert!(raw_links[next].is_some());
            raw_links[next].as_mut().unwrap().tail = prev;
            debug_assert!(extra_values.len() > prev);
            extra_values[prev].next = Link::Entry(next);
        }
        (Link::Extra(prev), Link::Extra(next)) => {
            debug_assert!(extra_values.len() > next);
            debug_assert!(extra_values.len() > prev);
            extra_values[prev].next = Link::Extra(next);
            extra_values[next].prev = Link::Extra(prev);
        }
    }
    let mut extra = extra_values.swap_remove(idx);
    let old_idx = extra_values.len();
    if extra.prev == Link::Extra(old_idx) {
        extra.prev = Link::Extra(idx);
    }
    if extra.next == Link::Extra(old_idx) {
        extra.next = Link::Extra(idx);
    }
    if idx != old_idx {
        let next;
        let prev;
        {
            debug_assert!(extra_values.len() > idx);
            let moved = &extra_values[idx];
            next = moved.next;
            prev = moved.prev;
        }
        match prev {
            Link::Entry(entry_idx) => {
                debug_assert!(raw_links[entry_idx].is_some());
                let links = raw_links[entry_idx].as_mut().unwrap();
                links.next = idx;
            }
            Link::Extra(extra_idx) => {
                debug_assert!(extra_values.len() > extra_idx);
                extra_values[extra_idx].next = Link::Extra(idx);
            }
        }
        match next {
            Link::Entry(entry_idx) => {
                debug_assert!(raw_links[entry_idx].is_some());
                let links = raw_links[entry_idx].as_mut().unwrap();
                links.tail = idx;
            }
            Link::Extra(extra_idx) => {
                debug_assert!(extra_values.len() > extra_idx);
                extra_values[extra_idx].prev = Link::Extra(idx);
            }
        }
    }
    debug_assert!(
        { for v in &* extra_values { assert!(v.next != Link::Extra(old_idx)); assert!(v
        .prev != Link::Extra(old_idx)); } true }
    );
    extra
}
