fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        if self.len() <= 1 {
            // There is no need to shuffle an empty or single element slice
            return;
        }
        self.partial_shuffle(rng, self.len());
    }