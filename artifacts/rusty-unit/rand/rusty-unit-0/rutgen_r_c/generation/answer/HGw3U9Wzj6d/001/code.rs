// Answer 0

#[test]
fn test_next_u64_valid() {
    struct RngImpl {
        counter: u64,
    }

    impl RngCore for RngImpl {
        fn next_u32(&mut self) -> u32 {
            (self.counter & 0xFFFFFFFF) as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.counter += 1;
            self.counter
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = self.next_u32() as u8;
            }
        }
    }

    let mut rng = RngImpl { counter: 0 };
    let value = rng.next_u64();
    assert_eq!(value, 1);
    let next_value = rng.next_u64();
    assert_eq!(next_value, 2);
}

#[test]
#[should_panic]
fn test_next_u64_panic() {
    struct RngImpl;

    impl RngCore for RngImpl {
        fn next_u32(&mut self) -> u32 {
            panic!("Panic in next_u32");
        }

        fn next_u64(&mut self) -> u64 {
            panic!("Panic in next_u64");
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            // No-op
        }
    }

    let mut rng = RngImpl;
    let _result = rng.next_u64();
}

