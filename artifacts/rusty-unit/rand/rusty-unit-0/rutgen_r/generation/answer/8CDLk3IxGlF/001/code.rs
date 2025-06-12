// Answer 0

#[test]
fn test_sample_valid_index() {
    struct TestSlice<'a, T> {
        slice: &'a [T],
        range: std::ops::Range<usize>,
    }

    impl<'a, T> TestSlice<'a, T> {
        fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> &'a T {
            let idx = self.range.start + rng.gen_range(0..(self.range.end - self.range.start));
            
            debug_assert!(
                idx < self.slice.len(),
                "Uniform::new(0, {}) somehow returned {}",
                self.slice.len(),
                idx
            );

            unsafe { self.slice.get_unchecked(idx) }
        }
    }

    let slice = [1, 2, 3, 4, 5];
    let range = 0..slice.len(); // Ensures 0 <= idx < slice.len()
    let test_slice = TestSlice { slice: &slice, range };

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let result = test_slice.sample(&mut rng);
        assert!(slice.contains(result), "Sampled value should be from the original slice.");
    }
}

#[should_panic]
#[test]
fn test_sample_panic_out_of_bounds() {
    struct TestSlice<'a, T> {
        slice: &'a [T],
        range: std::ops::Range<usize>,
    }

    impl<'a, T> TestSlice<'a, T> {
        fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> &'a T {
            let idx = self.range.start + rng.gen_range(0..(self.range.end - self.range.start));

            debug_assert!(
                idx < self.slice.len(),
                "Uniform::new(0, {}) somehow returned {}",
                self.slice.len(),
                idx
            );

            unsafe { self.slice.get_unchecked(idx) }
        }
    }

    let slice: &[i32] = &[]; // Empty slice should trigger panic
    let range = 0..1; // This range is invalid for the empty slice
    let test_slice = TestSlice { slice, range };

    let mut rng = rand::thread_rng();
    let _result = test_slice.sample(&mut rng);
}

