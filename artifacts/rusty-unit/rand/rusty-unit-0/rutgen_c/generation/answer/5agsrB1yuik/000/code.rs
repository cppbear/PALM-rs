// Answer 0

#[test]
fn test_fill_bytes() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }
    
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.results.clone());
        }
    }

    let core = MockBlockRngCore {
        results: vec![1, 2, 3, 4, 5],
    };
    
    let mut block_rng = BlockRng::new(core);
    let mut dest = [0u8; 16];
    
    block_rng.fill_bytes(&mut dest);
    
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_with_more_space() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }
    
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.results.clone());
        }
    }

    let core = MockBlockRngCore {
        results: vec![1, 2],
    };
    
    let mut block_rng = BlockRng::new(core);
    let mut dest = [0u8; 16];
    
    block_rng.fill_bytes(&mut dest);
    
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_with_exact_space() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }
    
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.results.clone());
        }
    }

    let core = MockBlockRngCore {
        results: vec![1, 2, 3],
    };
    
    let mut block_rng = BlockRng::new(core);
    let mut dest = [0u8; 12];
    
    block_rng.fill_bytes(&mut dest);
    
    assert_eq!(&dest, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_empty_dest() {
    struct MockBlockRngCore {
        results: Vec<u32>,
    }
    
    impl BlockRngCore for MockBlockRngCore {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.extend(self.results.clone());
        }
    }

    let core = MockBlockRngCore {
        results: vec![1, 2, 3],
    };
    
    let mut block_rng = BlockRng::new(core);
    let mut dest: [u8; 0] = [];
    
    block_rng.fill_bytes(&mut dest);
    
    assert_eq!(&dest, &[]);
}

