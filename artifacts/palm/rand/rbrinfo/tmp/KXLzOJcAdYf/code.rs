pub fn new<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error>
    where
        B1: SampleBorrow<X> + Sized,
        B2: SampleBorrow<X> + Sized,
    {
        X::Sampler::new(low, high).map(Uniform)
    }