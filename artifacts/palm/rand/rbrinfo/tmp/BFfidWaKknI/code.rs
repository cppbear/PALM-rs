fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        assert!(!range.is_empty(), "cannot sample empty range");
        range.sample_single(self).unwrap()
    }