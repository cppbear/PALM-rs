fn bitand(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A> {
        self.intersection(rhs).cloned().collect()
    }