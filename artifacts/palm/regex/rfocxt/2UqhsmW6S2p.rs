type InstPtr = u32;
type StatePtr = u32;
use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use std::mem;
use exec::ProgramCache;
use prog::{Inst, Program};
use sparse::SparseSet;
const STATE_UNKNOWN: StatePtr = 1 << 31;
const STATE_DEAD: StatePtr = STATE_UNKNOWN + 1;
const STATE_QUIT: StatePtr = STATE_DEAD + 1;
const STATE_START: StatePtr = 1 << 30;
const STATE_MATCH: StatePtr = 1 << 29;
const STATE_MAX: StatePtr = STATE_MATCH - 1;
#[derive(Clone, Debug)]
struct CacheInner {
    /// A cache of pre-compiled DFA states, keyed by the set of NFA states
    /// and the set of empty-width flags set at the byte in the input when the
    /// state was observed.
    ///
    /// A StatePtr is effectively a `*State`, but to avoid various inconvenient
    /// things, we just pass indexes around manually. The performance impact of
    /// this is probably an instruction or two in the inner loop. However, on
    /// 64 bit, each StatePtr is half the size of a *State.
    compiled: HashMap<State, StatePtr>,
    /// The transition table.
    ///
    /// The transition table is laid out in row-major order, where states are
    /// rows and the transitions for each state are columns. At a high level,
    /// given state `s` and byte `b`, the next state can be found at index
    /// `s * 256 + b`.
    ///
    /// This is, of course, a lie. A StatePtr is actually a pointer to the
    /// *start* of a row in this table. When indexing in the DFA's inner loop,
    /// this removes the need to multiply the StatePtr by the stride. Yes, it
    /// matters. This reduces the number of states we can store, but: the
    /// stride is rarely 256 since we define transitions in terms of
    /// *equivalence classes* of bytes. Each class corresponds to a set of
    /// bytes that never discriminate a distinct path through the DFA from each
    /// other.
    trans: Transitions,
    /// Our set of states. Note that `StatePtr / num_byte_classes` indexes
    /// this Vec rather than just a `StatePtr`.
    states: Vec<State>,
    /// A set of cached start states, which are limited to the number of
    /// permutations of flags set just before the initial byte of input. (The
    /// index into this vec is a `EmptyFlags`.)
    ///
    /// N.B. A start state can be "dead" (i.e., no possible match), so we
    /// represent it with a StatePtr.
    start_states: Vec<StatePtr>,
    /// Stack scratch space used to follow epsilon transitions in the NFA.
    /// (This permits us to avoid recursion.)
    ///
    /// The maximum stack size is the number of NFA states.
    stack: Vec<InstPtr>,
    /// The total number of times this cache has been flushed by the DFA
    /// because of space constraints.
    flush_count: u64,
    /// The total heap size of the DFA's cache. We use this to determine when
    /// we should flush the cache.
    size: usize,
}
#[derive(Clone)]
struct Transitions {
    /// The table.
    table: Vec<StatePtr>,
    /// The stride.
    num_byte_classes: usize,
}
#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    data: Box<[u8]>,
}
impl CacheInner {
    fn reset_size(&mut self) {
        self.size = (self.start_states.len() * mem::size_of::<StatePtr>())
            + (self.stack.len() * mem::size_of::<InstPtr>());
    }
}
