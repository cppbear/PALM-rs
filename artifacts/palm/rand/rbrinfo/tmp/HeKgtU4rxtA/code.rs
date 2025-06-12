fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        let mut x = self.sampler.sample(rng);
        if x >= CHAR_SURROGATE_START {
            x += CHAR_SURROGATE_LEN;
        }
        // SAFETY: x must not be in surrogate range or greater than char::MAX.
        // This relies on range constructors which accept char arguments.
        // Validity of input char values is assumed.
        unsafe { core::char::from_u32_unchecked(x) }
    }