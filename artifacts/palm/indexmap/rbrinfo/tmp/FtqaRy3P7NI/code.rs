pub fn difference<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Difference<'a, T, S2>
    where
        S2: BuildHasher,
    {
        Difference::new(self, other)
    }