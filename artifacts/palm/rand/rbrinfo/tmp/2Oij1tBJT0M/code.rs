fn choose_weighted<R, F, B, X>(
        &self,
        rng: &mut R,
        weight: F,
    ) -> Result<&Self::Output, WeightError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Output) -> B,
        B: SampleBorrow<X>,
        X: SampleUniform + Weight + PartialOrd<X>,
    {
        use crate::distr::{weighted::WeightedIndex, Distribution};
        let distr = WeightedIndex::new((0..self.len()).map(|idx| weight(&self[idx])))?;
        Ok(&self[distr.sample(rng)])
    }