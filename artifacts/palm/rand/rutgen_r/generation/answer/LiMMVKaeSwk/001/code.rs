// Answer 0

#[test]
fn test_next_u64() {
    struct TestRng(u64);

    impl TestRng {
        fn next_u64(&mut self) -> u64 {
            self.0 += 1;
            self.0
        }
    }

    let mut rng = TestRng(0);
    assert_eq!(rng.next_u64(), 1);
    assert_eq!(rng.next_u64(), 2);
    assert_eq!(rng.next_u64(), 3);
    // Test boundary condition
    rng.0 = u64::MAX - 1; // Set to maximum minus one to check boundary behavior
    assert_eq!(rng.next_u64(), u64::MAX);
}

