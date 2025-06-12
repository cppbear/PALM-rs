fn eq(&self, other: &IndexSet<T, S2>) -> bool {
        self.len() == other.len() && self.is_subset(other)
    }