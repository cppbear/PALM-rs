fn bitxor(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.symmetric_difference(other).cloned().collect()
    }