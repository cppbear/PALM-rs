pub fn is_subset(&self, other: &Self) -> bool {
        self.len() <= other.len() && self.iter().all(|v| other.contains(v))
    }