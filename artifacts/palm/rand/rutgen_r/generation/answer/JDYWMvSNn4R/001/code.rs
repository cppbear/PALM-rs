// Answer 0

#[test]
fn test_next_u32_bounds() {
    struct MockCore {
        calls: usize,
    }

    impl MockCore {
        fn generate(&mut self, results: &mut Vec<u32>) {
            self.calls += 1;
            results.push(0); // Push a value so we can test the output
        }
    }

    struct RandBlock {
        core: MockCore,
        results: Vec<u32>,
        index: usize,
        half_used: bool,
    }

    impl RandBlock {
        fn new(core: MockCore) -> Self {
            RandBlock {
                core,
                results: Vec::new(),
                index: 0,
                half_used: false,
            }
        }

        fn next_u32(&mut self) -> u32 {
            let mut index = self.index - self.half_used as usize;
            if index >= self.results.as_ref().len() {
                self.core.generate(&mut self.results);
                self.index = 0;
                index = 0;
                self.half_used = false;
            }

            let shift = 32 * (self.half_used as usize);

            self.half_used = !self.half_used;
            self.index += self.half_used as usize;

            (self.results.as_ref()[index] >> shift) as u32
        }
    }

    let mut core = MockCore { calls: 0 };
    let mut rand_block = RandBlock::new(core);

    // Set index to be equal to the length of `results`, which is initially 0
    rand_block.index = 0; // This means index will equal the length of results on first call

    let result = rand_block.next_u32();
    
    // At this point, the core should have been called once to generate results
    assert_eq!(core.calls, 1);
    assert_eq!(result, 0); // Since we pushed 0 in generate function, output should be 0
}

#[test]
fn test_next_u32_alternate_calls() {
    struct MockCore {
        calls: usize,
    }

    impl MockCore {
        fn generate(&mut self, results: &mut Vec<u32>) {
            self.calls += 1;
            results.push(42); // Push a fixed value to test output
        }
    }

    struct RandBlock {
        core: MockCore,
        results: Vec<u32>,
        index: usize,
        half_used: bool,
    }

    impl RandBlock {
        fn new(core: MockCore) -> Self {
            RandBlock {
                core,
                results: Vec::new(),
                index: 0,
                half_used: false,
            }
        }

        fn next_u32(&mut self) -> u32 {
            let mut index = self.index - self.half_used as usize;
            if index >= self.results.as_ref().len() {
                self.core.generate(&mut self.results);
                self.index = 0;
                index = 0;
                self.half_used = false;
            }

            let shift = 32 * (self.half_used as usize);

            self.half_used = !self.half_used;
            self.index += self.half_used as usize;

            (self.results.as_ref()[index] >> shift) as u32
        }
    }

    let mut core = MockCore { calls: 0 };
    let mut rand_block = RandBlock::new(core);

    // First call to next_u32 should trigger generate
    let first_result = rand_block.next_u32();
    assert_eq!(core.calls, 1);
    assert_eq!(first_result, 0); // The value pushed was 42, we expect output to be 0 shifted (lower half)

    // Second call to next_u32 should return the shifted value
    let second_result = rand_block.next_u32();
    assert_eq!(core.calls, 1); // Generate should not be called again
    assert_eq!(second_result, 42); // Now we shift the upper half, should return 42
}

