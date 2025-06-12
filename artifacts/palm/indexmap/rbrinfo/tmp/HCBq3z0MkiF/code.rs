fn eq(&self, other: &Slice<U>) -> bool {
        slice_eq(&self.entries, &other.entries, |b1, b2| b1.key == b2.key)
    }