fn into_iter(self) -> IndexVecIntoIter {
        match self {
            IndexVec::U32(v) => IndexVecIntoIter::U32(v.into_iter()),
            #[cfg(target_pointer_width = "64")]
            IndexVec::U64(v) => IndexVecIntoIter::U64(v.into_iter()),
        }
    }