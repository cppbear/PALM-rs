fn bitor_assign(&mut self, rhs: &HashSet<T, S, A>) {
        for item in rhs {
            if !self.contains(item) {
                self.insert(item.clone());
            }
        }
    }