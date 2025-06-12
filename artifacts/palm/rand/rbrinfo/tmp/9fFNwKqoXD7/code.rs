fn next(&mut self) -> Option<Self::Item> {
        use self::IndexVecIntoIter::*;
        match self {
            U32(v) => v.next().map(|i| i as usize),
            #[cfg(target_pointer_width = "64")]
            U64(v) => v.next().map(|i| i as usize),
        }
    }