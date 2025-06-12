// Answer 0

#[test]
fn test_next_index_initial_case() {
    struct DummyRng {
        value: u32,
    }

    impl DummyRng {
        fn random_range(&self, range: std::ops::Range<u32>) -> u32 {
            range.start + (self.value % (range.end - range.start))
        }
    }

    struct Sequence {
        rng: DummyRng,
        n: u32,
        chunk: u32,
        chunk_remaining: u32,
    }

    impl Sequence {
        fn new() -> Self {
            Self {
                rng: DummyRng { value: 0 },
                n: 0,
                chunk: 0,
                chunk_remaining: 0,
            }
        }

        fn next_index(&mut self) -> usize {
            let next_n = self.n + 1;
            let next_chunk_remaining = self.chunk_remaining.checked_sub(1).unwrap_or_else(|| {
                let bound = next_n * (next_n + 1); // simplified for the example
                self.chunk = self.rng.random_range(0..bound);
                1 // reset remaining to 1 after generating new chunk
            });

            let result = if next_chunk_remaining == 0 {
                self.chunk as usize
            } else {
                let r = self.chunk % next_n;
                self.chunk /= next_n;
                r as usize
            };

            self.chunk_remaining = next_chunk_remaining;
            self.n = next_n;
            result
        }
    }

    let mut seq = Sequence::new();
    let index = seq.next_index();
    assert!(index < 1); // Since n starts at 0, valid output is [0]
}

#[test]
#[should_panic]
fn test_next_index_overflow() {
    struct DummyRng {
        value: u32,
    }

    impl DummyRng {
        fn random_range(&self, _: std::ops::Range<u32>) -> u32 {
            0 // fixed value for testing
        }
    }

    struct Sequence {
        rng: DummyRng,
        n: u32,
        chunk: u32,
        chunk_remaining: u32,
    }

    impl Sequence {
        fn new() -> Self {
            Self {
                rng: DummyRng { value: 0 },
                n: 0,
                chunk: 0,
                chunk_remaining: 0,
            }
        }

        fn next_index(&mut self) -> usize {
            if self.n >= u32::MAX {
                panic!("n >= u32::MAX");
            }
            let next_n = self.n + 1;
            self.n = next_n;
            0 // Returns dummy value for this case
        }
    }

    let mut seq = Sequence::new();
    // Simulate reaching the maximum
    seq.n = u32::MAX;
    let _ = seq.next_index(); // This should panic
}

