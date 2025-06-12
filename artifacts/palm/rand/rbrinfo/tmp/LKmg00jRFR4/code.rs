fn random<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        StandardUniform.sample(self)
    }