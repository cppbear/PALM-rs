fn eq(&self, other: &Slice<K2, V2>) -> bool {
        slice_eq(&self.entries, &other.entries, |b1, b2| {
            b1.key == b2.key && b1.value == b2.value
        })
    }