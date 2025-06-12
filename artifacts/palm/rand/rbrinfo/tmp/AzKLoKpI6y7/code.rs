fn partial_shuffle<R>(&mut self, rng: &mut R, amount: usize) -> (&mut [T], &mut [T])
    where
        R: Rng + ?Sized,
    {
        let m = self.len().saturating_sub(amount);

        // The algorithm below is based on Durstenfeld's algorithm for the
        // [Fisher–Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle#The_modern_algorithm)
        // for an unbiased permutation.
        // It ensures that the last `amount` elements of the slice
        // are randomly selected from the whole slice.

        // `IncreasingUniform::next_index()` is faster than `Rng::random_range`
        // but only works for 32 bit integers
        // So we must use the slow method if the slice is longer than that.
        if self.len() < (u32::MAX as usize) {
            let mut chooser = IncreasingUniform::new(rng, m as u32);
            for i in m..self.len() {
                let index = chooser.next_index();
                self.swap(i, index);
            }
        } else {
            for i in m..self.len() {
                let index = rng.random_range(..i + 1);
                self.swap(i, index);
            }
        }
        let r = self.split_at_mut(m);
        (r.1, r.0)
    }