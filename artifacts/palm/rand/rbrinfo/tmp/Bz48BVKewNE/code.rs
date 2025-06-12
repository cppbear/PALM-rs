fn choose_mut<R>(&mut self, rng: &mut R) -> Option<&mut Self::Output>
    where
        R: Rng + ?Sized,
    {
        if self.is_empty() {
            None
        } else {
            let len = self.len();
            Some(&mut self[rng.random_range(..len)])
        }
    }