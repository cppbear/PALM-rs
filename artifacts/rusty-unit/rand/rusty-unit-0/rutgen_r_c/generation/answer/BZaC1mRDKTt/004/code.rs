// Answer 0

#[test]
fn test_random_ratio_true() {
    use rand::rngs::OsRng; // or any RNG implementation you prefer
    use crate::RngCore;

    struct TestRng {
        value: u32,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value += 1;
            self.value
        }
        
        fn next_u64(&mut self) -> u64 {
            self.next_u32() as u64
        }
        
        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
            unimplemented!()
        }
    }

    let mut rng = TestRng { value: 0 };
    let mut flipper = CoinFlipper::new(rng);

    // Test case where n < d, but we ensure c > 1 and flip_c_heads returns false
    let n = 2;
    let d = 8; // d is greater than n

    // Simulate flip_c_heads returning false
    flipper.chunk = 0b0000_0000_0000_0000_0000_0000_0000_0000; // This will ensure that leading zeros are enough to guarantee c > 1
    flipper.chunk_remaining = 32; // Reset chunk remaining for the test case 

    // Call the function and assert the expected outcome
    let result = flipper.random_ratio(n, d);
    assert_eq!(result, true);
}

#[test]
#[should_panic]
fn test_random_ratio_panic_on_next_n_zero() {
    use rand::rngs::OsRng; // or any RNG implementation you prefer
    use crate::RngCore;

    struct TestRng {
        value: u32,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value += 1;
            self.value
        }

        fn next_u64(&mut self) -> u64 {
            self.next_u32() as u64
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            unimplemented!()
        }

        fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
            unimplemented!()
        }
    }

    let mut rng = TestRng { value: 0 };
    let mut flipper = CoinFlipper::new(rng);

    // Prepare conditions to ensure next_n calculates to 0
    let n = 1;
    let d = 2;

    // Simulate flip_c_heads returning false
    flipper.chunk = 0b0000_0000_0000_0000_0000_0000_0000_0000; // This will ensure that leading zeros are extensive
    flipper.chunk_remaining = 32; // Reset chunk remaining for the test case

    // This should panic, as next_n will become 0
    let _result = flipper.random_ratio(n, d);
}

