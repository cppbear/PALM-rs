// Answer 0

#[test]
fn test_fill_bytes_empty_dest() {
    struct TestRng {
        state: u128,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            (self.state & 0xFFFFFFFF) as u32
        }
        
        fn next_u64(&mut self) -> u64 {
            (self.state & 0xFFFFFFFFFFFFFFFF) as u64
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRng { state: 12345 };
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
    assert_eq!(dest.len(), 0);
}

#[test]
fn test_fill_bytes_small_dest() {
    struct TestRng {
        state: u128,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            (self.state & 0xFFFFFFFF) as u32
        }
        
        fn next_u64(&mut self) -> u64 {
            (self.state & 0xFFFFFFFFFFFFFFFF) as u64
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRng { state: 12345 };
    let mut dest: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut dest);
    assert_ne!(dest, [0; 4]); // Check that we filled the bytes with something non-zero
}

#[test]
fn test_fill_bytes_large_dest() {
    struct TestRng {
        state: u128,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            (self.state & 0xFFFFFFFF) as u32
        }
        
        fn next_u64(&mut self) -> u64 {
            (self.state & 0xFFFFFFFFFFFFFFFF) as u64
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            impls::fill_bytes_via_next(self, dest)
        }
    }

    let mut rng = TestRng { state: 12345 };
    let mut dest: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut dest);
    assert_ne!(dest, [0; 16]); // Check that we filled the bytes with something
}

