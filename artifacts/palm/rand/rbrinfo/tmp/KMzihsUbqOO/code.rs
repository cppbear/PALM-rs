fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IndexVecIter::U32(v) => v.size_hint(),
            #[cfg(target_pointer_width = "64")]
            IndexVecIter::U64(v) => v.size_hint(),
        }
    }