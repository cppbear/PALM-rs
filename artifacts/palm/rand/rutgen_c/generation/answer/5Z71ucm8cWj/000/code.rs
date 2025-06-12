// Answer 0

#[test]
fn test_block_rng_reset() {
    struct TestBlockRng {
        results: Vec<u32>,
    }

    impl AsRef<[u32]> for TestBlockRng {
        fn as_ref(&self) -> &[u32] {
            &self.results
        }
    }

    impl AsMut<[u32]> for TestBlockRng {
        fn as_mut(&mut self) -> &mut [u32] {
            &mut self.results
        }
    }

    impl Default for TestBlockRng {
        fn default() -> Self {
            TestBlockRng { results: vec![1, 2, 3, 4, 5] }
        }
    }

    impl BlockRngCore for TestBlockRng {
        type Item = u32;
        type Results = TestBlockRng;

        fn generate(&mut self, _results: &mut Self::Results) {
            // Dummy implementation, does not actually generate
        }
    }

    let core = TestBlockRng::default();
    let mut block_rng = BlockRng::new(core);
    
    block_rng.reset();
    assert_eq!(block_rng.index(), block_rng.results.as_ref().len());
}

