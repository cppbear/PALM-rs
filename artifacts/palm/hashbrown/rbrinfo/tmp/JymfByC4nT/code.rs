fn sub_assign(&mut self, rhs: &HashSet<T, S, A>) {
        if rhs.len() < self.len() {
            for item in rhs {
                self.remove(item);
            }
        } else {
            self.retain(|item| !rhs.contains(item));
        }
    }