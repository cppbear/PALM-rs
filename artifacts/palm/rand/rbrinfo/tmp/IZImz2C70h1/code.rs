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
                Self::sample_single_inclusive(low, high - 1, rng)
            }