pub fn len(&self) -> usize {
        match self {
            IndexVec::U32(v) => v.len(),
            #[cfg(target_pointer_width = "64")]
            IndexVec::U64(v) => v.len(),
        }
    }