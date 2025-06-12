pub fn union<'a, S2>(&'a self, other: &'a IndexSet<T, S2>) -> Union<'a, T, S>
    where
        S2: BuildHasher,
    {
        Union::new(self, other)
    }