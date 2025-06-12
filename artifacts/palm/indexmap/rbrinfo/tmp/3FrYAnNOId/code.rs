pub(super) fn new(iter: vec::Drain<'a, Bucket<K, V>>) -> Self {
        Self { iter }
    }