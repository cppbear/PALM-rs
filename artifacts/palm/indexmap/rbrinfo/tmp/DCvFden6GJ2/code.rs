pub fn is_superset<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {
        other.is_subset(self)
    }