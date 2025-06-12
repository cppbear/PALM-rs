fn sample_single<R: Rng + ?Sized, B1, B2>(
        low_b: B1,
        high_b: B2,
        rng: &mut R,
    ) -> Result<Self::X, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low < high) {
            return Err(Error::EmptyRange);
        }

        if cfg!(target_pointer_width = "64") && high > (u32::MAX as usize) {
            return UniformInt::<u64>::sample_single(low as u64, high as u64, rng)
                .map(|x| x as usize);
        }

        UniformInt::<u32>::sample_single(low as u32, high as u32, rng).map(|x| x as usize)
    }