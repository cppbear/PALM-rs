fn partial_cmp(&self, other: &Char) -> Option<Ordering> {
        (*self as u32).partial_cmp(&other.0)
    }