fn sample<R: crate::Rng + ?Sized>(&self, rng: &mut R) -> &'a T {
        let idx = self.range.sample(rng);

        debug_assert!(
            idx < self.slice.len(),
            "Uniform::new(0, {}) somehow returned {}",
            self.slice.len(),
            idx
        );

        // Safety: at construction time, it was ensured that the slice was
        // non-empty, and that the `Uniform` range produces values in range
        // for the slice
        unsafe { self.slice.get_unchecked(idx) }
    }