fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                #[cfg(debug_assertions)]
                if !(low.all_finite()) || !(high.all_finite()) {
                    return Err(Error::NonFinite);
                }
                if !(low.all_lt(high)) {
                    return Err(Error::EmptyRange);
                }

                let scale = high - low;
                if !(scale.all_finite()) {
                    return Err(Error::NonFinite);
                }

                Ok(Self::new_bounded(low, high, scale))
            }