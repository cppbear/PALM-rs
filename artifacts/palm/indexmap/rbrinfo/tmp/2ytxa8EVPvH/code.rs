fn bitor(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.union(other).cloned().collect()
    }