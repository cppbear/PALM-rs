pub fn symmetric_difference<'a, S2>(
        &'a self,
        other: &'a IndexSet<T, S2>,
    ) -> SymmetricDifference<'a, T, S, S2>
    where
        S2: BuildHasher,
    {
        SymmetricDifference::new(self, other)
    }