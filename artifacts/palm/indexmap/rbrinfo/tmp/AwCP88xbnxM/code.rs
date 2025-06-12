fn eq(&self, other: &[U]) -> bool {
        slice_eq(&self.entries, other, |b, o| b.key == *o)
    }