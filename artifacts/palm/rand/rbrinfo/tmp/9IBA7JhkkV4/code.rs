fn size_hint(&self) -> (usize, Option<usize>) {
        use self::IndexVecIntoIter::*;
        match self {
            U32(v) => v.size_hint(),
            #[cfg(target_pointer_width = "64")]
            U64(v) => v.size_hint(),
        }
    }