fn partial_cmp(&self, other: &Literal) -> Option<cmp::Ordering> {
        self.v.partial_cmp(&other.v)
    }