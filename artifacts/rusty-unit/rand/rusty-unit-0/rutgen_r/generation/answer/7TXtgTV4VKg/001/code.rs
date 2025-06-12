// Answer 0

#[test]
fn test_next_u64_valid_case() {
    struct TestGenerator {
        results: Vec<u32>,
        index: usize,
    }

    impl TestGenerator {
        fn new(results: Vec<u32>) -> Self {
            Self { results, index: 0 }
        }

        fn generate_and_set(&mut self, count: usize) {
            self.results.extend(vec![0u32; count]);
        }

        fn next_u64(&mut self) -> u64 {
            let read_u64 = |results: &[u32], index| {
                let data = &results[index..=index + 1];
                (u64::from(data[1]) << 32) | u64::from(data[0])
            };

            let len = self.results.len();
            let index = self.index;
            if index < len - 1 {
                self.index += 2;
                read_u64(self.results.as_ref(), index)
            } else if index >= len {
                self.generate_and_set(2);
                read_u64(self.results.as_ref(), 0)
            } else {
                let x = u64::from(self.results.as_ref()[len - 1]);
                self.generate_and_set(1);
                let y = u64::from(self.results.as_ref()[0]);
                (y << 32) | x
            }
        }
    }

    let mut generator = TestGenerator::new(vec![1, 2, 3, 4]); // len is 4, initial index is 0
    assert_eq!(generator.next_u64(), (2u64 << 32) | 1); // reads from index 0
    assert_eq!(generator.index, 2); // should have incremented index by 2
    assert_eq!(generator.next_u64(), (4u64 << 32) | 3); // reads from index 2
    assert_eq!(generator.index, 4); // index should be out of bounds

    // Edge case when index is out of bounds
    assert_eq!(generator.next_u64(), (2u64 << 32) | 0); // triggers generate_and_set
    assert_eq!(generator.index, 2); // index resets after generating
}

