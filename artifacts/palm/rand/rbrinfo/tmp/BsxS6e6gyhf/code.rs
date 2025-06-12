pub fn iter(&self) -> IndexVecIter<'_> {
        match self {
            IndexVec::U32(v) => IndexVecIter::U32(v.iter()),
            #[cfg(target_pointer_width = "64")]
            IndexVec::U64(v) => IndexVecIter::U64(v.iter()),
        }
    }