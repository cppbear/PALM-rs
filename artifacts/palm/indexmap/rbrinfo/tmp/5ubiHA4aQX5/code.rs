pub(super) fn new(iter: vec::Drain<'a, Bucket<T>>) -> Self {
        Self { iter }
    }