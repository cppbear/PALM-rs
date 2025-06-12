// Answer 0

#[test]
fn test_fill_bytes() {
    struct TestRng {
        state: u128,
        increment: u128,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.state = self.state.wrapping_add(self.increment);
            (self.state as u32).wrapping_mul(MULTIPLIER as u32)
        }

        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(self.increment);
            (self.state as u64).wrapping_mul(MULTIPLIER)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRng { 
        state: 1, 
        increment: 1 
    };
    let mut bytes = [0u8; 16];
    
    rng.fill_bytes(&mut bytes);
    
    assert_eq!(bytes.len(), 16);
}

#[test]
#[should_panic]
fn test_fill_bytes_panic() {
    struct TestRng {
        state: u128,
        increment: u128,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.state = self.state.wrapping_add(self.increment);
            (self.state as u32).wrapping_mul(MULTIPLIER as u32)
        }

        fn next_u64(&mut self) -> u64 {
            self.state = self.state.wrapping_add(self.increment);
            (self.state as u64).wrapping_mul(MULTIPLIER)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRng { 
        state: 1, 
        increment: 1 
    };
    let mut bytes = [0u8; 16];

    // This will trigger panic if the internal logic of fill_bytes or its dependency fails.
    rng.fill_bytes(&mut bytes);
    panic!("Intentional panic for testing");
}

