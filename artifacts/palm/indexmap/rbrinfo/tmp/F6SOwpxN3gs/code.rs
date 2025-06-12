fn from(other: OccupiedEntry<'a, K, V>) -> Self {
        Self {
            index: other.index(),
            map: other.into_ref_mut(),
        }
    }