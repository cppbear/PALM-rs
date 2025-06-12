pub(super) fn new<S1>(set: &'a IndexSet<T, S1>, other: &'a IndexSet<T, S>) -> Self {
        Self {
            iter: set.iter(),
            other,
        }
    }