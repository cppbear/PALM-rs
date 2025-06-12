fn random_iter<T>(self) -> distr::Iter<StandardUniform, Self, T>
    where
        Self: Sized,
        StandardUniform: Distribution<T>,
    {
        StandardUniform.sample_iter(self)
    }