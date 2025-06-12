pub fn is_empty(&self) -> bool {
        match self {
            IndexVec::U32(v) => v.is_empty(),
            #[cfg(target_pointer_width = "64")]
            IndexVec::U64(v) => v.is_empty(),
        }
    }