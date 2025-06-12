// Answer 0

#[test]
fn test_from_rng_with_valid_rng() {
    struct MockRng {
        counter: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.counter += 1;
            self.counter
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

    struct MockBlockRng;

    impl BlockRngCore for MockBlockRng {
        type Item = u8;
        type Results = Vec<u8>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend_from_slice(&[1, 2, 3, 4]);
        }
    }

    let mut mock_rng = MockRng { counter: 0 };
    let block_rng = BlockRng64::from_rng(&mut mock_rng);

    assert_eq!(block_rng.index, 0);
    assert_eq!(block_rng.results.as_ref(), &[1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_from_rng_with_empty_rng() {
    struct EmptyRng;

    impl RngCore for EmptyRng {
        fn next_u32(&mut self) -> u32 {
            panic!("rng cannot produce values");
        }

        fn next_u64(&mut self) -> u64 {
            panic!("rng cannot produce values");
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 0; // Simulate that rng fails to fill bytes.
            }
        }
    }

    struct MockBlockRng;

    impl BlockRngCore for MockBlockRng {
        type Item = u8;
        type Results = Vec<u8>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend_from_slice(&[1, 2, 3, 4]);
        }
    }

    let mut empty_rng = EmptyRng;
    BlockRng64::<MockBlockRng>::from_rng(&mut empty_rng);
}

