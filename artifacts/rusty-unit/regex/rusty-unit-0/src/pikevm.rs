// This module implements the Pike VM. That is, it guarantees linear time
// search of a regex on any text with memory use proportional to the size of
// the regex.
//
// It is equal in power to the backtracking engine in this crate, except the
// backtracking engine is typically faster on small regexes/texts at the
// expense of a bigger memory footprint.
//
// It can do more than the DFA can (specifically, record capture locations
// and execute Unicode word boundary assertions), but at a slower speed.
// Specifically, the Pike VM executes a DFA implicitly by repeatedly expanding
// epsilon transitions. That is, the Pike VM engine can be in multiple states
// at once where as the DFA is only ever in one state at a time.
//
// Therefore, the Pike VM is generally treated as the fallback when the other
// matching engines either aren't feasible to run or are insufficient.

use std::mem;

use crate::exec::ProgramCache;
use crate::input::{Input, InputAt};
use crate::prog::{InstPtr, Program};
use crate::re_trait::Slot;
use crate::sparse::SparseSet;

/// An NFA simulation matching engine.
#[derive(Debug)]
pub struct Fsm<'r, I> {
    /// The sequence of opcodes (among other things) that is actually executed.
    ///
    /// The program may be byte oriented or Unicode codepoint oriented.
    prog: &'r Program,
    /// An explicit stack used for following epsilon transitions. (This is
    /// borrowed from the cache.)
    stack: &'r mut Vec<FollowEpsilon>,
    /// The input to search.
    input: I,
}

/// A cached allocation that can be reused on each execution.
#[derive(Clone, Debug)]
pub struct Cache {
    /// A pair of ordered sets for tracking NFA states.
    clist: Threads,
    nlist: Threads,
    /// An explicit stack used for following epsilon transitions.
    stack: Vec<FollowEpsilon>,
}

/// An ordered set of NFA states and their captures.
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

/// A representation of an explicit stack frame when following epsilon
/// transitions. This is used to avoid recursion.
#[derive(Clone, Debug)]
enum FollowEpsilon {
    /// Follow transitions at the given instruction pointer.
    IP(InstPtr),
    /// Restore the capture slot with the given position in the input.
    Capture { slot: usize, pos: Slot },
}

impl Cache {
    /// Create a new allocation used by the NFA machine to record execution
    /// and captures.
    pub fn new(_prog: &Program) -> Self {
        Cache { clist: Threads::new(), nlist: Threads::new(), stack: vec![] }
    }
}

impl<'r, I: Input> Fsm<'r, I> {
    /// Execute the NFA matching engine.
    ///
    /// If there's a match, `exec` returns `true` and populates the given
    /// captures accordingly.
    pub fn exec(
        prog: &'r Program,
        cache: &ProgramCache,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        input: I,
        start: usize,
        end: usize,
    ) -> bool {
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.pikevm;
        cache.clist.resize(prog.len(), prog.captures.len());
        cache.nlist.resize(prog.len(), prog.captures.len());
        let at = input.at(start);
        Fsm { prog, stack: &mut cache.stack, input }.exec_(
            &mut cache.clist,
            &mut cache.nlist,
            matches,
            slots,
            quit_after_match,
            at,
            end,
        )
    }

    fn exec_(
        &mut self,
        mut clist: &mut Threads,
        mut nlist: &mut Threads,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        mut at: InputAt,
        end: usize,
    ) -> bool {
        let mut matched = false;
        let mut all_matched = false;
        clist.set.clear();
        nlist.set.clear();
        'LOOP: loop {
            if clist.set.is_empty() {
                // Three ways to bail out when our current set of threads is
                // empty.
                //
                // 1. We have a match---so we're done exploring any possible
                //    alternatives. Time to quit. (We can't do this if we're
                //    looking for matches for multiple regexes, unless we know
                //    they all matched.)
                //
                // 2. If the expression starts with a '^' we can terminate as
                //    soon as the last thread dies.
                if (matched && matches.len() <= 1)
                    || all_matched
                    || (!at.is_start() && self.prog.is_anchored_start)
                {
                    break;
                }

                // 3. If there's a literal prefix for the program, try to
                //    jump ahead quickly. If it can't be found, then we can
                //    bail out early.
                if !self.prog.prefixes.is_empty() {
                    at = match self.input.prefix_at(&self.prog.prefixes, at) {
                        None => break,
                        Some(at) => at,
                    };
                }
            }

            // This simulates a preceding '.*?' for every regex by adding
            // a state starting at the current position in the input for the
            // beginning of the program only if we don't already have a match.
            if clist.set.is_empty()
                || (!self.prog.is_anchored_start && !all_matched)
            {
                self.add(&mut clist, slots, 0, at);
            }
            // The previous call to "add" actually inspects the position just
            // before the current character. For stepping through the machine,
            // we can to look at the current character, so we advance the
            // input.
            let at_next = self.input.at(at.next_pos());
            for i in 0..clist.set.len() {
                let ip = clist.set[i];
                if self.step(
                    &mut nlist,
                    matches,
                    slots,
                    clist.caps(ip),
                    ip,
                    at,
                    at_next,
                ) {
                    matched = true;
                    all_matched = all_matched || matches.iter().all(|&b| b);
                    if quit_after_match {
                        // If we only care if a match occurs (not its
                        // position), then we can quit right now.
                        break 'LOOP;
                    }
                    if self.prog.matches.len() == 1 {
                        // We don't need to check the rest of the threads
                        // in this set because we've matched something
                        // ("leftmost-first"). However, we still need to check
                        // threads in the next set to support things like
                        // greedy matching.
                        //
                        // This is only true on normal regexes. For regex sets,
                        // we need to mush on to observe other matches.
                        break;
                    }
                }
            }
            if at.pos() >= end {
                break;
            }
            at = at_next;
            mem::swap(clist, nlist);
            nlist.set.clear();
        }
        matched
    }

    /// Step through the input, one token (byte or codepoint) at a time.
    ///
    /// nlist is the set of states that will be processed on the next token
    /// in the input.
    ///
    /// caps is the set of captures passed by the caller of the NFA. They are
    /// written to only when a match state is visited.
    ///
    /// thread_caps is the set of captures set for the current NFA state, ip.
    ///
    /// at and at_next are the current and next positions in the input. at or
    /// at_next may be EOF.
    fn step(
        &mut self,
        nlist: &mut Threads,
        matches: &mut [bool],
        slots: &mut [Slot],
        thread_caps: &mut [Option<usize>],
        ip: usize,
        at: InputAt,
        at_next: InputAt,
    ) -> bool {
        use crate::prog::Inst::*;
        match self.prog[ip] {
            Match(match_slot) => {
                if match_slot < matches.len() {
                    matches[match_slot] = true;
                }
                for (slot, val) in slots.iter_mut().zip(thread_caps.iter()) {
                    *slot = *val;
                }
                true
            }
            Char(ref inst) => {
                if inst.c == at.char() {
                    self.add(nlist, thread_caps, inst.goto, at_next);
                }
                false
            }
            Ranges(ref inst) => {
                if inst.matches(at.char()) {
                    self.add(nlist, thread_caps, inst.goto, at_next);
                }
                false
            }
            Bytes(ref inst) => {
                if let Some(b) = at.byte() {
                    if inst.matches(b) {
                        self.add(nlist, thread_caps, inst.goto, at_next);
                    }
                }
                false
            }
            EmptyLook(_) | Save(_) | Split(_) => false,
        }
    }

    /// Follows epsilon transitions and adds them for processing to nlist,
    /// starting at and including ip.
    fn add(
        &mut self,
        nlist: &mut Threads,
        thread_caps: &mut [Option<usize>],
        ip: usize,
        at: InputAt,
    ) {
        self.stack.push(FollowEpsilon::IP(ip));
        while let Some(frame) = self.stack.pop() {
            match frame {
                FollowEpsilon::IP(ip) => {
                    self.add_step(nlist, thread_caps, ip, at);
                }
                FollowEpsilon::Capture { slot, pos } => {
                    thread_caps[slot] = pos;
                }
            }
        }
    }

    /// A helper function for add that avoids excessive pushing to the stack.
    fn add_step(
        &mut self,
        nlist: &mut Threads,
        thread_caps: &mut [Option<usize>],
        mut ip: usize,
        at: InputAt,
    ) {
        // Instead of pushing and popping to the stack, we mutate ip as we
        // traverse the set of states. We only push to the stack when we
        // absolutely need recursion (restoring captures or following a
        // branch).
        use crate::prog::Inst::*;
        loop {
            // Don't visit states we've already added.
            if nlist.set.contains(ip) {
                return;
            }
            nlist.set.insert(ip);
            match self.prog[ip] {
                EmptyLook(ref inst) => {
                    if self.input.is_empty_match(at, inst) {
                        ip = inst.goto;
                    }
                }
                Save(ref inst) => {
                    if inst.slot < thread_caps.len() {
                        self.stack.push(FollowEpsilon::Capture {
                            slot: inst.slot,
                            pos: thread_caps[inst.slot],
                        });
                        thread_caps[inst.slot] = Some(at.pos());
                    }
                    ip = inst.goto;
                }
                Split(ref inst) => {
                    self.stack.push(FollowEpsilon::IP(inst.goto2));
                    ip = inst.goto1;
                }
                Match(_) | Char(_) | Ranges(_) | Bytes(_) => {
                    let t = &mut nlist.caps(ip);
                    for (slot, val) in t.iter_mut().zip(thread_caps.iter()) {
                        *slot = *val;
                    }
                    return;
                }
            }
        }
    }
}

impl Threads {
    fn new() -> Self {
        Threads { set: SparseSet::new(0), caps: vec![], slots_per_thread: 0 }
    }

    fn resize(&mut self, num_insts: usize, ncaps: usize) {
        if num_insts == self.set.capacity() {
            return;
        }
        self.slots_per_thread = ncaps * 2;
        self.set = SparseSet::new(num_insts);
        self.caps = vec![None; self.slots_per_thread * num_insts];
    }

    fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
        let i = pc * self.slots_per_thread;
        &mut self.caps[i..i + self.slots_per_thread]
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::Ord;
	use std::default::Default;
	use std::clone::Clone;
	use std::convert::From;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut usize_0: usize = 5684usize;
    let mut str_0: &str = "5OIC73MkV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut regexset_0: crate::re_set::unicode::RegexSet = crate::re_set::unicode::RegexSet::empty();
    let mut regexset_0_ref_0: &crate::re_set::unicode::RegexSet = &mut regexset_0;
    let mut setmatches_0: crate::re_set::unicode::SetMatches = crate::re_set::unicode::RegexSet::matches(regexset_0_ref_0, str_0_ref_0);
    let mut setmatches_0_ref_0: &crate::re_set::unicode::SetMatches = &mut setmatches_0;
    let mut setmatchesiter_0: crate::re_set::unicode::SetMatchesIter = crate::re_set::unicode::SetMatches::iter(setmatches_0_ref_0);
    let mut setmatchesiter_0_ref_0: &crate::re_set::unicode::SetMatchesIter = &mut setmatchesiter_0;
    let mut usize_1: usize = 291usize;
    let mut error_0: error::Error = crate::error::Error::CompiledTooBig(usize_1);
    let mut error_0_ref_0: &error::Error = &mut error_0;
    let mut program_0: crate::prog::Program = crate::prog::Program::new();
    let mut program_0_ref_0: &crate::prog::Program = &mut program_0;
    let mut bool_0: bool = false;
    let mut str_1: &str = "8";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut threads_0: crate::pikevm::Threads = crate::pikevm::Threads::new();
    let mut threads_0_ref_0: &crate::pikevm::Threads = &mut threads_0;
    let mut usize_2: usize = 8846usize;
    let mut usize_3: usize = 1827usize;
    let mut sparseset_0: crate::sparse::SparseSet = crate::sparse::SparseSet::new(usize_3);
    let mut threads_1: crate::pikevm::Threads = crate::pikevm::Threads::clone(threads_0_ref_0);
    let mut regexbuilder_0: crate::re_builder::bytes::RegexBuilder = crate::re_builder::bytes::RegexBuilder::new(str_1_ref_0);
    let mut cache_0: crate::pikevm::Cache = crate::pikevm::Cache::new(program_0_ref_0);
    let mut sparseset_0_ref_0: &crate::sparse::SparseSet = &mut sparseset_0;
    let mut usize_4: usize = crate::sparse::SparseSet::len(sparseset_0_ref_0);
    let mut error_1: error::Error = crate::error::Error::clone(error_0_ref_0);
    let mut threads_1_ref_0: &crate::pikevm::Threads = &mut threads_1;
    let mut threads_2: crate::pikevm::Threads = crate::pikevm::Threads::clone(threads_1_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_46() {
//     rusty_monitor::set_test_id(46);
//     let mut char_0: char = 'k';
//     let mut char_1: crate::input::Char = crate::input::Char::from(char_0);
//     let mut char_1_ref_0: &crate::input::Char = &mut char_1;
//     let mut usize_0: usize = 7300usize;
//     let mut usize_1: usize = 3352usize;
//     let mut bool_0: bool = true;
//     let mut bool_1: bool = false;
//     let mut bool_2: bool = true;
//     let mut bool_3: bool = true;
//     let mut char_2: char = 'j';
//     let mut char_3: crate::input::Char = crate::input::Char::from(char_2);
//     let mut char_3_ref_0: &crate::input::Char = &mut char_3;
//     let mut usize_2: usize = 9348usize;
//     let mut bool_4: bool = false;
//     let mut bool_5: bool = true;
//     let mut usize_3: usize = 8856usize;
//     let mut usize_4: usize = 7715usize;
//     let mut usize_5: usize = 3785usize;
//     let mut bool_6: bool = true;
//     let mut bool_7: bool = true;
//     let mut usize_6: usize = 8152usize;
//     let mut bool_8: bool = true;
//     let mut bool_9: bool = true;
//     let mut usize_7: usize = 2561usize;
//     let mut bool_10: bool = true;
//     let mut bool_11: bool = true;
//     let mut ordering_0: std::cmp::Ordering = crate::input::Char::cmp(char_3_ref_0, char_1_ref_0);
//     let mut regexset_0: crate::re_set::bytes::RegexSet = crate::re_set::bytes::RegexSet::empty();
//     let mut suffixcachekey_0: crate::compile::SuffixCacheKey = crate::compile::SuffixCacheKey::default();
//     let mut regexset_0_ref_0: &crate::re_set::bytes::RegexSet = &mut regexset_0;
//     panic!("From RustyUnit with love");
// }
}