fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        // A valid `char` is either in the interval `[0, 0xD800)` or
        // `(0xDFFF, 0x11_0000)`. All `char`s must therefore be in
        // `[0, 0x11_0000)` but not in the "gap" `[0xD800, 0xDFFF]` which is
        // reserved for surrogates. This is the size of that gap.
        const GAP_SIZE: u32 = 0xDFFF - 0xD800 + 1;

        // Uniform::new(0, 0x11_0000 - GAP_SIZE) can also be used, but it
        // seemed slower.
        let range = Uniform::new(GAP_SIZE, 0x11_0000).unwrap();

        let mut n = range.sample(rng);
        if n <= 0xDFFF {
            n -= GAP_SIZE;
        }
        // SAFETY: We ensure above that `n` represents a `char`.
        unsafe { char::from_u32_unchecked(n) }
    }