fn eq(&self, other: &[(K2, V2)]) -> bool {
        slice_eq(&self.entries, other, |b, t| b.key == t.0 && b.value == t.1)
    }