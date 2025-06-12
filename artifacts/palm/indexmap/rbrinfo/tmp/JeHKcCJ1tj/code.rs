pub fn is_disjoint<S2>(&self, other: &IndexSet<T, S2>) -> bool
    where
        S2: BuildHasher,
    {
        if self.len() <= other.len() {
            self.iter().all(move |value| !other.contains(value))
        } else {
            other.iter().all(move |value| !self.contains(value))
        }
    }