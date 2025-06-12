// Answer 0

#[test]
fn test_fill_bytes() {
    struct MockRng {
        v: u64,
        a: u64,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.v = self.v.wrapping_add(self.a);
            self.v as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.v = self.v.wrapping_add(self.a);
            self.v
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            impls::fill_bytes_via_next(self, dst)
        }
    }

    let mut rng = MockRng { v: 1, a: 1 };
    let mut buffer = [0u8; 16];
    
    rng.fill_bytes(&mut buffer);

    assert_ne!(buffer, [0u8; 16]);
    assert_eq!(buffer.len(), 16);
}

