fn eq(&self, other: &IndexVec) -> bool {
        use self::IndexVec::*;
        match (self, other) {
            (U32(v1), U32(v2)) => v1 == v2,
            #[cfg(target_pointer_width = "64")]
            (U64(v1), U64(v2)) => v1 == v2,
            #[cfg(target_pointer_width = "64")]
            (U32(v1), U64(v2)) => {
                (v1.len() == v2.len()) && (v1.iter().zip(v2.iter()).all(|(x, y)| *x as u64 == *y))
            }
            #[cfg(target_pointer_width = "64")]
            (U64(v1), U32(v2)) => {
                (v1.len() == v2.len()) && (v1.iter().zip(v2.iter()).all(|(x, y)| *x == *y as u64))
            }
        }
    }