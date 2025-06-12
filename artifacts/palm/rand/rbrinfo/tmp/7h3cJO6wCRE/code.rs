pub fn index(&self, index: usize) -> usize {
        match self {
            IndexVec::U32(v) => v[index] as usize,
            #[cfg(target_pointer_width = "64")]
            IndexVec::U64(v) => v[index] as usize,
        }
    }