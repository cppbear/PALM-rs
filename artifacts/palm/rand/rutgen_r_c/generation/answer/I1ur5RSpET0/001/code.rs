// Answer 0

#[test]
fn test_from_rng_valid() {
    struct MockRng {
        current: u32,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.current;
            self.current += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            self.next_u32() as u64
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = self.next_u32() as u8;
            }
        }
    }

    struct MockBlockRngCore {
        data: [u8; 16],
    }

    impl Default for MockBlockRngCore {
        fn default() -> Self {
            Self { data: [0; 16] }
        }
    }

    impl AsRef<[u8]> for MockBlockRngCore {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    impl AsMut<[u8]> for MockBlockRngCore {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u8;
        type Results = MockBlockRngCore;

        fn generate(&mut self, results: &mut Self::Results) {
            results.data.copy_from_slice(&self.data);
        }
    }

    let mut rng = MockRng { current: 0 };
    let block_rng = BlockRng64::from_rng(&mut rng);
    assert_eq!(block_rng.index, 0);
}

#[test]
#[should_panic]
fn test_from_rng_empty_rng() {
    struct PanicRng;

    impl RngCore for PanicRng {
        fn next_u32(&mut self) -> u32 {
            panic!("This RNG should not be called");
        }

        fn next_u64(&mut self) -> u64 {
            panic!("This RNG should not be called");
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            panic!("This RNG should not be called");
        }
    }

    let mut rng = PanicRng;
    let _ = BlockRng64::from_rng(&mut rng);
}

