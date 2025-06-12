// Answer 0

#[test]
fn test_fill_bytes_with_filled_dest() {
    struct DummyBlockRng {
        index: usize,
        results: Vec<u32>,
    }

    impl BlockRngCore for DummyBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..results.len() {
                results[i] = self.index as u32 + i as u32;
            }
            self.index += results.len();
        }
    }

    let mut core = DummyBlockRng {
        index: 0,
        results: vec![0; 10],
    };
    let mut block_rng = BlockRng::new(core);

    let mut dest = [0u8; 40]; // 10 u32 * 4 bytes each
    block_rng.fill_bytes(&mut dest);

    assert_eq!(dest, [
        0, 0, 0, 0,
        1, 0, 0, 0,
        2, 0, 0, 0,
        3, 0, 0, 0,
        4, 0, 0, 0,
        5, 0, 0, 0,
        6, 0, 0, 0,
        7, 0, 0, 0,
        8, 0, 0, 0,
        9, 0, 0, 0,
    ]);
}

#[test]
fn test_fill_bytes_with_empty_dest() {
    struct DummyBlockRng {
        index: usize,
        results: Vec<u32>,
    }

    impl BlockRngCore for DummyBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..results.len() {
                results[i] = self.index as u32 + i as u32;
            }
            self.index += results.len();
        }
    }

    let core = DummyBlockRng {
        index: 0,
        results: vec![0; 10],
    };
    let mut block_rng = BlockRng::new(core);

    let mut dest: [u8; 0] = [];
    block_rng.fill_bytes(&mut dest); // should not panic

    assert_eq!(dest.len(), 0, "Destination should remain empty");
}

#[test]
#[should_panic]
fn test_fill_bytes_with_panic_on_index() {
    struct DummyBlockRng {
        index: usize,
        results: Vec<u32>,
    }

    impl BlockRngCore for DummyBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..results.len() {
                results[i] = self.index as u32 + i as u32;
            }
            self.index += results.len();
        }
    }

    let mut core = DummyBlockRng {
        index: 10, // set index greater than results length
        results: vec![0; 10],
    };
    let mut block_rng = BlockRng::new(core);

    let mut dest = [0u8; 40]; 
    block_rng.fill_bytes(&mut dest); // Panic expected here
}

#[test]
#[should_panic]
fn test_fill_bytes_with_bound_panic_on_dest() {
    struct DummyBlockRng {
        index: usize,
        results: Vec<u32>,
    }

    impl BlockRngCore for DummyBlockRng {
        type Item = u32;
        type Results = Vec<u32>;

        fn generate(&mut self, results: &mut Self::Results) {
            for i in 0..results.len() {
                results[i] = self.index as u32 + i as u32;
            }
            self.index += results.len();
        }
    }

    let mut core = DummyBlockRng {
        index: 0,
        results: vec![0; 10],
    };
    let mut block_rng = BlockRng::new(core);

    let mut dest = [0u8; 5]; // Dest len < needed to fill
    block_rng.fill_bytes(&mut dest); // Panic expected here
}

