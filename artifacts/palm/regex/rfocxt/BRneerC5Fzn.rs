use std::ops::Deref;
use std::slice;
#[derive(Clone, Debug)]
pub struct SparseSet {
    /// Dense contains the instruction pointers in the order in which they
    /// were inserted. Accessing elements >= self.size is illegal.
    dense: Vec<usize>,
    /// Sparse maps instruction pointers to their location in dense.
    ///
    /// An instruction pointer is in the set if and only if
    /// sparse[ip] < size && ip == dense[sparse[ip]].
    sparse: Vec<usize>,
    /// The number of elements in the set.
    size: usize,
}
impl SparseSet {
    pub fn new(size: usize) -> SparseSet {
        SparseSet {
            dense: vec![0; size],
            sparse: vec![0; size],
            size: 0,
        }
    }
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn capacity(&self) -> usize {}
    pub fn insert(&mut self, value: usize) {}
    pub fn contains(&self, value: usize) -> bool {}
    pub fn clear(&mut self) {}
}
