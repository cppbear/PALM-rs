use std::mem;
use exec::ProgramCache;
use input::{Input, InputAt};
use prog::{Program, InstPtr};
use re_trait::Slot;
use sparse::SparseSet;
#[derive(Clone, Debug)]
struct Threads {
    /// An ordered set of opcodes (each opcode is an NFA state).
    set: SparseSet,
    /// Captures for every NFA state.
    ///
    /// It is stored in row-major order, where the columns are the capture
    /// slots and the rows are the states.
    caps: Vec<Slot>,
    /// The number of capture slots stored per thread. (Every capture has
    /// two slots.)
    slots_per_thread: usize,
}
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
impl Threads {
    fn new() -> Self {
        Threads {
            set: SparseSet::new(0),
            caps: vec![],
            slots_per_thread: 0,
        }
    }
    fn resize(&mut self, num_insts: usize, ncaps: usize) {}
    fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {}
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
