pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }