// Answer 0

#[test]
fn test_next_u32_with_generated_results() {
    struct MockCore {
        count: usize,
    }

    impl MockCore {
        fn generate(&mut self, results: &mut Vec<u32>) {
            if self.count < 3 {
                results.push(0x12345678);
                results.push(0x87654321);
                results.push(0xFFFFFFFF);
                self.count += 1;
            }
        }
    }

    struct RandomGenerator {
        core: MockCore,
        results: Vec<u32>,
        index: usize,
        half_used: bool,
    }

    impl RandomGenerator {
        fn new(core: MockCore) -> Self {
            Self {
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

    let mut core = MockCore { count: 0 };
    let mut generator = RandomGenerator::new(core);

    let value1 = generator.next_u32();
    assert_eq!(value1, 0x12345678); // First call gives first result

    let value2 = generator.next_u32();
    assert_eq!(value2, 0x87654321); // Second call gives second result

    let value3 = generator.next_u32();
    assert_eq!(value3, 0xFFFFFFFF); // Third call gives third result

    let value4 = generator.next_u32();
    assert_eq!(value4, 0x12345678 >> 32); // Fourth call wraps around, gives first result shifted
}

