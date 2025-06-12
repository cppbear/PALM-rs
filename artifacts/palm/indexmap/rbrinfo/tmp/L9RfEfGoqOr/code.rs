pub(super) fn new<S2>(set1: &'a IndexSet<T, S>, set2: &'a IndexSet<T, S2>) -> Self
    where
        S2: BuildHasher,
    {
        Self {
            iter: set1.iter().chain(set2.difference(set1)),
        }
    }