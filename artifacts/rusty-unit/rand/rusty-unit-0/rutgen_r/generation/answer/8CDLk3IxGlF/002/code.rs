// Answer 0

#[derive(Debug)]
struct SliceWrapper<'a, T> {
    slice: &'a [T],
    range: std::ops::Range<usize>,
}

impl<'a, T> SliceWrapper<'a, T> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand::distributions::Uniform;

    #[test]
    #[should_panic(expected = "Uniform::new(0, 1) somehow returned 1")]
    fn test_sample_panic() {
        let slice = [1];
        let uniform_range = Uniform::new(0, 1); // Bound idx to create a panic condition
        let mut rng = rand::thread_rng();

        let wrapper = SliceWrapper {
            slice: &slice,
            range: uniform_range,
        };

        wrapper.sample(&mut rng);
    }

    #[test]
    fn test_sample_valid() {
        let slice = [10, 20, 30];
        let uniform_range = Uniform::new(0, 3); // Valid range for indices 0 to 2
        let mut rng = rand::thread_rng();

        let wrapper = SliceWrapper {
            slice: &slice,
            range: uniform_range,
        };

        let result = wrapper.sample(&mut rng);
        assert!(slice.contains(result)); // Check that the result is within the slice
    }
}

