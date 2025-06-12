fn intersect(&self, other: &Self) -> Option<Self> {
        let lower = cmp::max(self.lower(), other.lower());
        let upper = cmp::min(self.upper(), other.upper());
        if lower <= upper {
            Some(Self::create(lower, upper))
        } else {
            None
        }
    }