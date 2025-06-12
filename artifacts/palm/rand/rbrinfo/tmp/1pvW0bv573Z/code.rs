fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = char_to_comp_u32(*low_b.borrow());
        let high = char_to_comp_u32(*high_b.borrow());
        let sampler = UniformInt::<u32>::new(low, high);
        sampler.map(|sampler| UniformChar { sampler })
    }