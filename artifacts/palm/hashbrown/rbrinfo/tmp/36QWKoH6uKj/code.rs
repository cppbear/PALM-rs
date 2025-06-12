fn sub(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A> {
        self.difference(rhs).cloned().collect()
    }