unsafe fn next_si(&self, si: StatePtr, text: &[u8], i: usize) -> StatePtr {
        // What is the argument for safety here?
        // We have three unchecked accesses that could possibly violate safety:
        //
        //   1. The given byte of input (`text[i]`).
        //   2. The class of the byte of input (`classes[text[i]]`).
        //   3. The transition for the class (`trans[si + cls]`).
        //
        // (1) is only safe when calling next_si is guarded by
        // `i < text.len()`.
        //
        // (2) is the easiest case to guarantee since `text[i]` is always a
        // `u8` and `self.prog.byte_classes` always has length `u8::MAX`.
        // (See `ByteClassSet.byte_classes` in `compile.rs`.)
        //
        // (3) is only safe if (1)+(2) are safe. Namely, the transitions
        // of every state are defined to have length equal to the number of
        // byte classes in the program. Therefore, a valid class leads to a
        // valid transition. (All possible transitions are valid lookups, even
        // if it points to a state that hasn't been computed yet.) (3) also
        // relies on `si` being correct, but StatePtrs should only ever be
        // retrieved from the transition table, which ensures they are correct.
        debug_assert!(i < text.len());
        let b = *text.get_unchecked(i);
        debug_assert!((b as usize) < self.prog.byte_classes.len());
        let cls = *self.prog.byte_classes.get_unchecked(b as usize);
        self.cache.trans.next_unchecked(si, cls as usize)
    }