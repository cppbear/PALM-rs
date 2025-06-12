fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low < high) {
                    return Err(Error::EmptyRange);
                }
                UniformSampler::new_inclusive(low, high - 1)
            }