fn sub(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.difference(other).cloned().collect()
    }