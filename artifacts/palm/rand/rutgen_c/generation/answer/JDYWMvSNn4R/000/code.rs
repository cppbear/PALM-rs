// Answer 0

#[test]
fn test_next_u32() {
    struct TestCore {
        count: usize,
        results: Vec<u64>,
    }
    
    impl BlockRngCore for TestCore {
        type Item = u64;
        type Results = Vec<u64>;

        fn generate(&mut self, results: &mut Self::Results) {
            results.clear();
            for _ in 0..2 {
                results.push(self.count as u64);
                self.count += 1;
            }
        }
    }

    let core = TestCore { count: 0, results: vec![] };
    let mut rng = BlockRng64 {
        results: vec![0, 0],
        index: 0,
        half_used: false,
        core,
    };

    assert_eq!(rng.next_u32(), 0); // First call, first half of results[0]: 0
    assert_eq!(rng.next_u32(), 0); // Second call, second half of results[0]: 0
    assert_eq!(rng.next_u32(), 1); // Third call, first half of results[1]: 1
    assert_eq!(rng.next_u32(), 1); // Fourth call, second half of results[1]: 1
    assert_eq!(rng.next_u32(), 2); // Fifth call, results need to be regenerated
}

