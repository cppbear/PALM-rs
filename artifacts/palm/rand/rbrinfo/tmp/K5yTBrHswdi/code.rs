fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        #[cfg(target_pointer_width = "32")]
        let mode32 = true;
        #[cfg(target_pointer_width = "64")]
        let mode32 = !self.mode64;

        if mode32 {
            let range = self.range as u32;
            if range == 0 {
                return rng.random::<u32>() as usize;
            }

            let thresh = self.thresh as u32;
            let hi = loop {
                let (hi, lo) = rng.random::<u32>().wmul(range);
                if lo >= thresh {
                    break hi;
                }
            };
            self.low.wrapping_add(hi as usize)
        } else {
            let range = self.range as u64;
            if range == 0 {
                return rng.random::<u64>() as usize;
            }

            let thresh = self.thresh as u64;
            let hi = loop {
                let (hi, lo) = rng.random::<u64>().wmul(range);
                if lo >= thresh {
                    break hi;
                }
            };
            self.low.wrapping_add(hi as usize)
        }
    }