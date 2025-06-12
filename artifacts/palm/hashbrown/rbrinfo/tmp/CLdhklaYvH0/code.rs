pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
    }