fn bitxor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A> {
        self.symmetric_difference(rhs).cloned().collect()
    }