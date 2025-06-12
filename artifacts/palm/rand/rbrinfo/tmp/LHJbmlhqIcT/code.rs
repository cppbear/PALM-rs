fn next(&mut self) -> Option<usize> {
        use self::IndexVecIter::*;
        match self {
            U32(iter) => iter.next().map(|i| *i as usize),
            #[cfg(target_pointer_width = "64")]
            U64(iter) => iter.next().map(|i| *i as usize),
        }
    }