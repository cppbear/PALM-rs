// Answer 0

#[test]
fn test_generate_and_set_valid_index() {
    struct MockRngCore;
    
    impl RngCore for MockRngCore {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 42;
            }
        }
    }

    struct MockBlockRngCore {
        results: [u32; 10],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 10];

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..results.len() {
                results[i] = self.results[i];
            }
        }
    }

    let mut block_rng = BlockRng64::new(MockBlockRngCore { results: [0; 10] });
    block_rng.generate_and_set(5);

    assert_eq!(block_rng.index(), 5);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_generate_and_set_invalid_index() {
    struct MockRngCore;
    
    impl RngCore for MockRngCore {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = 42;
            }
        }
    }

    struct MockBlockRngCore {
        results: [u32; 3],
    }

    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = [u32; 3];

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..results.len() {
                results[i] = self.results[i];
            }
        }
    }

    let mut block_rng = BlockRng64::new(MockBlockRngCore { results: [0; 3] });
    block_rng.generate_and_set(5);
}

