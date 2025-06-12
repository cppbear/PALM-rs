fn bitor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A> {
        self.union(rhs).cloned().collect()
    }