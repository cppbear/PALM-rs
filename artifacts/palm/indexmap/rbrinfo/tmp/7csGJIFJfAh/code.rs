fn eq(&self, other: &Slice<U>) -> bool {
        slice_eq(self, &other.entries, |o, b| *o == b.key)
    }