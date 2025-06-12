use std::fmt;
use std::ops::Deref;
use std::slice;

/// A sparse set used for representing ordered NFA states.
///
/// This supports constant time addition and membership testing. Clearing an
/// entire set can also be done in constant time. Iteration yields elements
/// in the order in which they were inserted.
///
/// The data structure is based on: https://research.swtch.com/sparse
/// Note though that we don't actually use uninitialized memory. We generally
/// reuse allocations, so the initial allocation cost is bareable. However,
/// its other properties listed above are extremely useful.
#[derive(Clone)]
pub struct SparseSet {
    /// Dense contains the instruction pointers in the order in which they
    /// were inserted.
    dense: Vec<usize>,
    /// Sparse maps instruction pointers to their location in dense.
    ///
    /// An instruction pointer is in the set if and only if
    /// sparse[ip] < dense.len() && ip == dense[sparse[ip]].
    sparse: Box<[usize]>,
}

impl SparseSet {
    pub fn new(size: usize) -> SparseSet {
        SparseSet {
            dense: Vec::with_capacity(size),
            sparse: vec![0; size].into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.dense.capacity()
    }

    pub fn insert(&mut self, value: usize) {
        let i = self.len();
        assert!(i < self.capacity());
        self.dense.push(value);
        self.sparse[value] = i;
    }

    pub fn contains(&self, value: usize) -> bool {
        let i = self.sparse[value];
        self.dense.get(i) == Some(&value)
    }

    pub fn clear(&mut self) {
        self.dense.clear();
    }
}

impl fmt::Debug for SparseSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SparseSet({:?})", self.dense)
    }
}

impl Deref for SparseSet {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.dense
    }
}

impl<'a> IntoIterator for &'a SparseSet {
    type Item = &'a usize;
    type IntoIter = slice::Iter<'a, usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests_llm_16_202 {
    use super::*;

use crate::*;
    use std::ops::Deref;

    #[test]
    fn test_deref() {
        let mut sparse_set = SparseSet::new(10);
        sparse_set.insert(1);
        sparse_set.insert(2);
        sparse_set.insert(3);

        let dense: &[usize] = sparse_set.deref();
        assert_eq!(dense, &[1, 2, 3]);
    }

    #[test]
    fn test_deref_empty() {
        let sparse_set = SparseSet::new(5);
        let dense: &[usize] = sparse_set.deref();
        assert!(dense.is_empty());
    }

    #[test]
    fn test_deref_after_clear() {
        let mut sparse_set = SparseSet::new(5);
        sparse_set.insert(1);
        sparse_set.insert(2);
        sparse_set.clear();

        let dense: &[usize] = sparse_set.deref();
        assert!(dense.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_660 {
    use super::*;

use crate::*;
    use crate::sparse::SparseSet;

    #[test]
    fn test_capacity() {
        let sparse_set = SparseSet::new(10);
        assert_eq!(sparse_set.capacity(), 10);
        
        let mut sparse_set_with_items = SparseSet::new(5);
        sparse_set_with_items.insert(1);
        sparse_set_with_items.insert(2);
        assert_eq!(sparse_set_with_items.capacity(), 5);
    }
}

#[cfg(test)]
mod tests_llm_16_661 {
    use super::*;

use crate::*;
    use crate::sparse::SparseSet;

    #[test]
    fn test_clear() {
        let mut sparse_set = SparseSet::new(5);
        sparse_set.insert(1);
        sparse_set.insert(2);
        sparse_set.insert(3);

        assert_eq!(sparse_set.len(), 3);
        assert!(!sparse_set.is_empty());

        sparse_set.clear();

        assert_eq!(sparse_set.len(), 0);
        assert!(sparse_set.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_662 {
    use super::*;

use crate::*;
    use crate::sparse::SparseSet;

    #[test]
    fn test_contains() {
        let mut sparse_set = SparseSet::new(10);
        sparse_set.insert(1);
        sparse_set.insert(5);
        sparse_set.insert(3);
        
        assert!(sparse_set.contains(1));
        assert!(sparse_set.contains(5));
        assert!(!sparse_set.contains(2));
        assert!(!sparse_set.contains(4));
        assert!(sparse_set.contains(3));
    }

    #[test]
    fn test_contains_empty_set() {
        let sparse_set = SparseSet::new(10);
        assert!(!sparse_set.contains(0));
    }

    #[test]
    fn test_contains_out_of_bounds() {
        let mut sparse_set = SparseSet::new(5);
        sparse_set.insert(0);
        
        // The behavior for out of bounds must be considered; here we expect panic
        // We use a Result to test for panic.
        let result = std::panic::catch_unwind(|| {
            sparse_set.contains(10); // out of bounds value
        });
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_llm_16_664 {
    use super::*;

use crate::*;
    use std::ops::Deref;

    #[test]
    fn test_is_empty() {
        let sparse_set = SparseSet::new(10);
        assert!(sparse_set.is_empty());

        let mut sparse_set_filled = SparseSet::new(10);
        sparse_set_filled.insert(1);
        assert!(!sparse_set_filled.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_666 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_sparse_set_new() {
        let size = 10;
        let sparse_set = SparseSet::new(size);
        
        assert_eq!(sparse_set.len(), 0);
        assert_eq!(sparse_set.capacity(), size);
        assert!(sparse_set.is_empty());
    }
}
