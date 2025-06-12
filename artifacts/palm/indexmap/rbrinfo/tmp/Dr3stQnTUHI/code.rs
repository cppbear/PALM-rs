fn eq(&self, other: &Slice<K2, V2>) -> bool {
        slice_eq(self, &other.entries, |t, b| t.0 == b.key && t.1 == b.value)
    }