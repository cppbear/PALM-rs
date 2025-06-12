// Answer 0

#[test]
fn test_next_u64() {
    use rand_core::{RngCore, SeedableRng};
    use rand_core::CryptoRng;

    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.value & 0xFF) as u8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 16];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng {
                value: u64::from_ne_bytes(seed[0..8].try_into().unwrap()),
            }
        }
    }

    let rng = &mut TestRng { value: 42 };
    let result = rng.next_u64();
    assert_eq!(result, 42);
}

#[test]
fn test_next_u64_boundary() {
    struct BoundaryRng {
        value: u64,
    }

    impl RngCore for BoundaryRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.value & 0xFF) as u8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let rng = &mut BoundaryRng { value: u64::MAX };
    let result = rng.next_u64();
    assert_eq!(result, u64::MAX);
}

