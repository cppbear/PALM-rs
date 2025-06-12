// Answer 0

#[test]
fn test_next_u32() {
    struct TestRng {
        state: u128,
        increment: u128,
    }

    impl RngCore for TestRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            let result = (self.state.wrapping_mul(MULTIPLIER) + self.increment) as u64;
            self.state = self.state.wrapping_mul(MULTIPLIER) + self.increment;
            result
        }

        #[inline]
        fn fill_bytes(&mut self, _dest: &mut [u8]) {}
    }

    let mut rng = TestRng {
        state: 1,
        increment: 2,
    };

    assert_eq!(rng.next_u32(), (1.wrapping_mul(MULTIPLIER) + 2) as u32);
    rng.state += 1; // Change state to test further output
    assert_eq!(rng.next_u32(), (2.wrapping_mul(MULTIPLIER) + 2) as u32);
    rng.state += 1; // Change state again
    assert_eq!(rng.next_u32(), (3.wrapping_mul(MULTIPLIER) + 2) as u32);
}

