pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.entries
            .partition_point(move |a| pred(&a.key, &a.value))
    }