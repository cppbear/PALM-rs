fn bitand_assign(&mut self, rhs: &HashSet<T, S, A>) {
        self.retain(|item| rhs.contains(item));
    }