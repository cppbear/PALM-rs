fn choose_weighted_mut<R, F, B, X>(
        &mut self,
        rng: &mut R,
        weight: F,
    ) -> Result<&mut Self::Output, WeightError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Output) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform + Weight + PartialOrd<X>,
    {
        use crate::distr::{weighted::WeightedIndex, Distribution};
        let distr = WeightedIndex::new((0..self.len()).map(|idx| weight(&self[idx])))?;
        let index = distr.sample(rng);
        Ok(&mut self[index])
    }