fn try_from_rng<S: TryRngCore>(rng: &mut S) -> Result<Self, S::Error> {
        R::try_from_rng(rng).map(Self::new)
    }