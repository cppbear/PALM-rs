fn bitand(self, other: &IndexSet<T, S2>) -> Self::Output {
        self.intersection(other).cloned().collect()
    }