// Answer 0

#[test]
fn test_next_index_zero_remaining() {
    struct RngStub {
        current: u32,
    }
    
    impl RngStub {
        fn random_range(&self, range: std::ops::Range<u32>) -> u32 {
            self.current % (range.end - range.start) + range.start
        }
    }

    struct Sequence {
        n: u32,
        chunk: u32,
        chunk_remaining: u32,
        rng: RngStub,
    }

    impl Sequence {
        fn new() -> Self {
            Sequence {
                n: 0,
                chunk: 0,
                chunk_remaining: 0,
                rng: RngStub { current: 0 },
            }
        }

        fn next_index(&mut self) -> usize {
            let next_n = self.n + 1;
            let next_chunk_remaining = self.chunk_remaining.checked_sub(1).unwrap_or_else(|| {
                let (bound, remaining) = (next_n + 1, 2); // Stub for calculate_bound_u32
                self.chunk = self.rng.random_range(0..bound);
                remaining - 1
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

    let mut sequence = Sequence::new();
    
    // Setting up an initial state where chunk_remaining becomes 0
    sequence.chunk_remaining = 0; 
    sequence.chunk = 4; // Random chunk value for testing
    let result = sequence.next_index();
    
    assert!(result < 1); // since n will be 1, return value must be in [0, 1)
}

#[test]
#[should_panic]
fn test_next_index_overflow() {
    struct RngStub {
        current: u32,
    }

    impl RngStub {
        fn random_range(&self, range: std::ops::Range<u32>) -> u32 {
            self.current % (range.end - range.start) + range.start
        }
    }

    struct Sequence {
        n: u32,
        chunk: u32,
        chunk_remaining: u32,
        rng: RngStub,
    }

    impl Sequence {
        fn new() -> Self {
            Sequence {
                n: u32::MAX,
                chunk: 0,
                chunk_remaining: 0,
                rng: RngStub { current: 0 },
            }
        }

        fn next_index(&mut self) -> usize {
            let next_n = self.n + 1; // This will overflow
            let next_chunk_remaining = self.chunk_remaining.checked_sub(1).unwrap_or_else(|| {
                let (bound, remaining) = (next_n + 1, 2); // Stub for calculate_bound_u32
                self.chunk = self.rng.random_range(0..bound);
                remaining - 1
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
    
    let mut sequence = Sequence::new();
    
    // This will trigger panic due to n reaching u32::MAX
    let _ = sequence.next_index();
}

