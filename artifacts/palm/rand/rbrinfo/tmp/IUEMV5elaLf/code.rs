pub fn into_vec(self) -> Vec<usize> {
        match self {
            IndexVec::U32(v) => v.into_iter().map(|i| i as usize).collect(),
            #[cfg(target_pointer_width = "64")]
            IndexVec::U64(v) => v.into_iter().map(|i| i as usize).collect(),
        }
    }