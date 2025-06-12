// Answer 0

#[test]
fn test_next_u32() {
    struct TestRng {
        lcg: Lcg128CmDxsm64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.lcg.next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            self.lcg.next_u64()
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.lcg.fill_bytes(dest);
        }
    }

    let mut rng = TestRng {
        lcg: Lcg128CmDxsm64 {
            state: 0x1234567890abcdef1234567890abcdef,
            increment: 0x1fedcba987654321fedcba9876543210,
        },
    };

    let result = rng.next_u32();
    assert_eq!(result, rng.lcg.next_u64() as u32);
}

#[test]
fn test_next_u32_boundary() {
    let mut rng = Lcg128CmDxsm64 {
        state: u128::MAX,
        increment: u128::MAX,
    };

    let result = rng.next_u32();
    assert!(result <= u32::MAX);
}

