fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low <= high) {
            return Err(Error::EmptyRange);
        }

        #[cfg(target_pointer_width = "64")]
        let mode64 = high > (u32::MAX as usize);
        #[cfg(target_pointer_width = "32")]
        let mode64 = false;

        let (range, thresh);
        if cfg!(target_pointer_width = "64") && !mode64 {
            let range32 = (high as u32).wrapping_sub(low as u32).wrapping_add(1);
            range = range32 as usize;
            thresh = if range32 > 0 {
                (range32.wrapping_neg() % range32) as usize
            } else {
                0
            };
        } else {
            range = high.wrapping_sub(low).wrapping_add(1);
            thresh = if range > 0 {
                range.wrapping_neg() % range
            } else {
                0
            };
        }

        Ok(UniformUsize {
            low,
            range,
            thresh,
            #[cfg(target_pointer_width = "64")]
            mode64,
        })
    }