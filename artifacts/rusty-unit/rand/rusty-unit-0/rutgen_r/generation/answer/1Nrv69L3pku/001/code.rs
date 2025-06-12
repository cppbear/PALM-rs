// Answer 0

#[test]
fn test_sample_valid_rng() {
    use rand::Rng;
    use rand::thread_rng;

    struct TestRng {
        state: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            // Cycle through a fixed range of values
            self.state = (self.state + 1) % 64; // Ensure it stays within [0, 63]
            self.state
        }

        fn gen_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            self.next_u32() % (range.end - range.start) + range.start
        }

        // other required methods can be no-ops
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
            Ok(())
        }
        fn clone(&self) -> Self {
            TestRng { state: self.state }
        }
    }

    let mut rng = TestRng { state: 0 };
    
    for _ in 0..100 {
        let result = sample(&(), &mut rng);
        assert!(result.is_ascii_alphanumeric(), "Returned value is not valid ASCII character");
    }
}

#[test]
#[should_panic]
fn test_sample_invalid_rng() {
    use rand::Rng;
    use rand::thread_rng;

    struct PanicRng;

    impl Rng for PanicRng {
        fn next_u32(&mut self) -> u32 {
            // This RNG does not produce valid values as desired
            100 // Outside the range expected
        }

        fn gen_range(&mut self, range: std::ops::Range<u32>) -> u32 {
            // Return some value outside of the valid range
            100
        }

        // other required methods can be no-ops
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
            Ok(())
        }
        fn clone(&self) -> Self {
            PanicRng
        }
    }

    let mut rng = PanicRng;
    let _ = sample(&(), &mut rng); // This should cause a panic due to invalid range.
}

