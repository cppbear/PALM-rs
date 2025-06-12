use super::{equivalent, Entries, IndexMapCore, RefMut};
use crate::HashValue;
use core::{fmt, mem};
use hashbrown::hash_table;
pub struct VacantEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    hash: HashValue,
    key: K,
}
#[repr(transparent)]
pub struct Slice<K, V> {
    pub(crate) entries: [Bucket<K, V>],
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct HashValue(usize);
#[derive(Copy, Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}
struct RefMut<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn index(&self) -> usize {}
    pub fn key(&self) -> &K {}
    pub(crate) fn key_mut(&mut self) -> &mut K {}
    pub fn into_key(self) -> K {}
    pub fn insert(self, value: V) -> &'a mut V {}
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {}
    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        let slice = crate::map::Slice::from_slice(self.map.entries);
        let i = slice.binary_search_keys(&self.key).unwrap_err();
        (i, self.shift_insert(i, value))
    }
    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map.shift_insert_unique(index, self.hash, self.key, value);
        &mut self.map.entries[index].value
    }
}
impl<K, V> Slice<K, V> {
    pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<K, V>> {}
    pub const fn new<'a>() -> &'a Self {
        Self::from_slice(&[])
    }
    pub fn new_mut<'a>() -> &'a mut Self {
        Self::from_mut_slice(&mut [])
    }
    #[inline]
    pub const fn len(&self) -> usize {}
    #[inline]
    pub const fn is_empty(&self) -> bool {}
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {}
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {}
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get(range).map(Slice::from_slice)
    }
    pub fn get_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> Option<&mut Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get_mut(range).map(Slice::from_mut_slice)
    }
    pub fn first(&self) -> Option<(&K, &V)> {}
    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn last(&self) -> Option<(&K, &V)> {}
    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {}
    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }
    pub fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
        let (first, second) = self.entries.split_at_mut(index);
        (Self::from_mut_slice(first), Self::from_mut_slice(second))
    }
    pub fn split_first(&self) -> Option<((&K, &V), &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((first.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [first, rest @ ..] = &mut self.entries {
            Some((first.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last(&self) -> Option<((&K, &V), &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((last.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }
    pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [rest @ .., last] = &mut self.entries {
            Some((last.ref_mut(), Self::from_mut_slice(rest)))
        } else {
            None
        }
    }
    pub fn iter(&self) -> Iter<'_, K, V> {}
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {}
    pub fn keys(&self) -> Keys<'_, K, V> {}
    pub fn into_keys(self: Box<Self>) -> IntoKeys<K, V> {}
    pub fn values(&self) -> Values<'_, K, V> {}
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {}
    pub fn into_values(self: Box<Self>) -> IntoValues<K, V> {}
    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.binary_search_by(|p, _| p.cmp(x))
    }
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> Ordering,
    {}
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        mut f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> B,
        B: Ord,
    {}
    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {}
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {}
    #[allow(unsafe_code)]
    pub(crate) fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {}
}
#[allow(unsafe_code)]
impl<K, V> Slice<K, V> {
    pub(super) const fn from_slice(entries: &[Bucket<K, V>]) -> &Self {
        unsafe { &*(entries as *const [Bucket<K, V>] as *const Self) }
    }
    pub(super) fn from_mut_slice(entries: &mut [Bucket<K, V>]) -> &mut Self {
        unsafe { &mut *(entries as *mut [Bucket<K, V>] as *mut Self) }
    }
    pub(super) fn from_boxed(entries: Box<[Bucket<K, V>]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(entries) as *mut Self) }
    }
    fn into_boxed(self: Box<Self>) -> Box<[Bucket<K, V>]> {}
}
