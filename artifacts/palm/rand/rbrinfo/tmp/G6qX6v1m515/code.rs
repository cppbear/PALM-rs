fn choose<R>(&self, rng: &mut R) -> Option<&Self::Output>
    where
        R: Rng + ?Sized,
    {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.random_range(..self.len())])
        }
    }