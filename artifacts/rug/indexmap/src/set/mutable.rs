use core::hash::{BuildHasher, Hash};

use super::{Equivalent, IndexSet};
use crate::map::MutableKeys;

/// Opt-in mutable access to [`IndexSet`] values.
///
/// These methods expose `&mut T`, mutable references to the value as it is stored
/// in the set.
/// You are allowed to modify the values in the set **if the modification
/// does not change the value’s hash and equality**.
///
/// If values are modified erroneously, you can no longer look them up.
/// This is sound (memory safe) but a logical error hazard (just like
/// implementing `PartialEq`, `Eq`, or `Hash` incorrectly would be).
///
/// `use` this trait to enable its methods for `IndexSet`.
///
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait MutableValues: private::Sealed {
    type Value;

    /// Return item index and mutable reference to the value
    ///
    /// Computes in **O(1)** time (average).
    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
    where
        Q: ?Sized + Hash + Equivalent<Self::Value>;

    /// Return mutable reference to the value at an index.
    ///
    /// Valid indices are `0 <= index < self.len()`.
    ///
    /// Computes in **O(1)** time.
    fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value>;

    /// Scan through each value in the set and keep those where the
    /// closure `keep` returns `true`.
    ///
    /// The values are visited in order, and remaining values keep their order.
    ///
    /// Computes in **O(n)** time (average).
    fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut Self::Value) -> bool;
}

/// Opt-in mutable access to [`IndexSet`] values.
///
/// See [`MutableValues`] for more information.
impl<T, S> MutableValues for IndexSet<T, S>
where
    S: BuildHasher,
{
    type Value = T;

    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        match self.map.get_full_mut2(value) {
            Some((index, value, ())) => Some((index, value)),
            None => None,
        }
    }

    fn get_index_mut2(&mut self, index: usize) -> Option<&mut T> {
        match self.map.get_index_mut2(index) {
            Some((value, ())) => Some(value),
            None => None,
        }
    }

    fn retain2<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.map.retain2(move |value, ()| keep(value));
    }
}

mod private {
    pub trait Sealed {}

    impl<T, S> Sealed for super::IndexSet<T, S> {}
}

#[cfg(test)]
mod tests_llm_16_214 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_get_full_mut2_found() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);
        
        let result = index_set.get_full_mut2(&2);
        assert!(result.is_some());
        let (index, value) = result.unwrap();
        assert_eq!(index, 1);
        assert_eq!(*value, 2);
    }

    #[test]
    fn test_get_full_mut2_not_found() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        
        let result = index_set.get_full_mut2(&3);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_full_mut2_modify() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        
        {
            let result = index_set.get_full_mut2(&1);
            assert!(result.is_some());
            let (_, value) = result.unwrap();
            *value *= 2;
        }

        // Verify modification
        assert_eq!(index_set.get(&1), Some(&2));
    }
}

#[cfg(test)]
mod tests_llm_16_216 {
    use super::*;

use crate::*;
    use crate::IndexMap;

    #[test]
    fn test_retain2() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        index_set.insert(1);
        index_set.insert(2);
        index_set.insert(3);
        
        index_set.retain2(|value| {
            // Retain only even numbers
            *value % 2 == 0
        });

        let retained: Vec<i32> = index_set.iter().copied().collect();
        assert_eq!(retained, vec![2]);
    }

    #[test]
    fn test_retain2_no_elements() {
        let mut index_set: IndexSet<i32> = IndexSet::new();
        
        index_set.retain2(|_value| {
            // Nothing to retain, as the index set is empty
            true
        });

        let retained: Vec<i32> = index_set.iter().copied().collect();
        assert!(retained.is_empty());
    }

    #[test]
    fn test_retain2_all_elements() {
        let mut index_set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3, 4]);
        
        index_set.retain2(|_value| {
            // Retain all, return true for all
            true
        });

        let retained: Vec<i32> = index_set.iter().copied().collect();
        assert_eq!(retained, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_retain2_no_elements_retained() {
        let mut index_set: IndexSet<i32> = IndexSet::from_iter(vec![1, 2, 3]);
        
        index_set.retain2(|value| {
            // Retain no elements
            *value > 3
        });

        let retained: Vec<i32> = index_set.iter().copied().collect();
        assert!(retained.is_empty());
    }
}
