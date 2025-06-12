// Answer 0

#[test]
fn test_block_rng64_from_seed() {
    struct TestRng {
        state: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 1;
            self.state as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.state += 1;
            self.state
        }

        fn fill_bytes(&mut self, dst: &mut [u8]) {
            for byte in dst.iter_mut() {
                *byte = (self.next_u8() % 256) as u8;
            }
        }

        fn next_u8(&mut self) -> u8 {
            (self.next_u32() % 256) as u8
        }
    }

    struct TestBlockRngCore {
        results: [u8; 16],
    }

    impl Default for TestBlockRngCore {
        fn default() -> Self {
            Self { results: [0; 16] }
        }
    }

    impl AsRef<[u8]> for TestBlockRngCore {
        fn as_ref(&self) -> &[u8] {
            &self.results
        }
    }

    impl AsMut<[u8]> for TestBlockRngCore {
        fn as_mut(&mut self) -> &mut [u8] {
            &mut self.results
        }
    }

    impl BlockRngCore for TestBlockRngCore {
        type Item = u8;
        type Results = Self;

        fn generate(&mut self, results: &mut Self::Results) {
            results.results.copy_from_slice(&self.results);
        }
    }

    impl SeedableRng for TestBlockRngCore {
        type Seed = [u8; 16];

        fn from_seed(seed: Self::Seed) -> Self {
            let mut core = Self::default();
            core.results.copy_from_slice(&seed);
            core
        }
    }

    let seed = [1u8; 16];
    let block_rng = BlockRng64::<TestBlockRngCore>::from_seed(seed.clone());
    
    assert_eq!(block_rng.core.results, seed);
}

