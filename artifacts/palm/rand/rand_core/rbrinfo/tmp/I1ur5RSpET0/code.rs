fn from_rng(rng: &mut impl RngCore) -> Self {
        Self::new(R::from_rng(rng))
    }