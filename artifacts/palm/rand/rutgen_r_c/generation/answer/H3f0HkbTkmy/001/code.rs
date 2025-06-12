// Answer 0

#[test]
fn test_sample_single_inclusive_valid_range() {
    struct MockRng;

    impl RngCore for MockRng {
        // Implement required methods for RngCore
        fn next_u32(&mut self) -> u32 {
            42 // Return a fixed value for testing
        }
        fn next_u64(&mut self) -> u64 {
            42 // Return a fixed value for testing
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(42); // Fill with a fixed byte value for consistency
        }
        fn throw_bytes(&mut self, _dest: &mut [u8]) {
            // No-op for the mock
        }
    }

    let mut rng = MockRng;
    let range = 1..=10; // Valid inclusive range
    let result: Result<u8, Error> = range.sample_single(&mut rng);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value >= 1 && value <= 10); // Assert the value is within the expected range
}

#[test]
#[should_panic]
fn test_sample_single_empty_range() {
    struct MockRng;

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            // No-op
        }
        fn throw_bytes(&mut self, _dest: &mut [u8]) {
            // No-op
        }
    }

    let mut rng = MockRng;
    let range: RangeInclusive<u8> = 1..=0; // Invalid range
    let _result: Result<u8, Error> = range.sample_single(&mut rng); // This should panic
}

#[test]
fn test_sample_single_with_finite_range() {
    struct MockRng;

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            1 // Return a deterministic value for testing
        }
        fn next_u64(&mut self) -> u64 {
            1 // Return a deterministic value for testing
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            // No-op
        }
        fn throw_bytes(&mut self, _dest: &mut [u8]) {
            // No-op
        }
    }

    let mut rng = MockRng;
    let range = 0..=20; // Valid finite range
    let result: Result<u32, Error> = range.sample_single(&mut rng);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value >= 0 && value <= 20); // Assert the value is within the expected range
} 

#[test]
#[should_panic]
fn test_sample_single_non_finite_range() {
    struct MockRng;

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            // No-op
        }
        fn throw_bytes(&mut self, _dest: &mut [u8]) {
            // No-op
        }
    }

    let mut rng = MockRng;
    let range: RangeInclusive<f64> = f64::NAN..=f64::INFINITY; // Non-finite range
    let _result: Result<f64, Error> = range.sample_single(&mut rng); // This should panic
}

