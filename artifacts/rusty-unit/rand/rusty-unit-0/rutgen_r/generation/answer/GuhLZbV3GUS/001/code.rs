// Answer 0

#[derive(Debug)]
struct MockGenerator {
    index: usize,
    results: Vec<u32>,
}

impl MockGenerator {
    fn new(results: Vec<u32>) -> Self {
        Self { index: 0, results }
    }

    fn next_u32(&mut self) -> u32 {
        if self.index >= self.results.len() {
            self.generate_and_set(0);
        }

        let value = self.results[self.index];
        self.index += 1;
        value
    }

    fn generate_and_set(&mut self, _seed: u32) {
        // For testing, we can add predefined values or simply extend the results
        self.results.push(1); // This is to avoid panics on the next call
    }
}

#[test]
fn test_next_u32_with_empty_results() {
    let mut generator = MockGenerator::new(vec![]);
    let value = generator.next_u32();
    assert_eq!(value, 1); // We expect the method to add a generated value of 1
}

#[test]
fn test_next_u32_with_one_element() {
    let mut generator = MockGenerator::new(vec![42]);
    let value = generator.next_u32();
    assert_eq!(value, 42);
}

#[test]
fn test_next_u32_exceed_index() {
    let mut generator = MockGenerator::new(vec![10, 20]);
    let first_value = generator.next_u32();
    let second_value = generator.next_u32();
    let third_value = generator.next_u32(); // This should trigger generation

    assert_eq!(first_value, 10);
    assert_eq!(second_value, 20);
    assert_eq!(third_value, 1); // Generated value after exhaustion
}

#[test]
fn test_next_u32_multiple_calls() {
    let mut generator = MockGenerator::new(vec![5, 10, 15]);
    assert_eq!(generator.next_u32(), 5);
    assert_eq!(generator.next_u32(), 10);
    assert_eq!(generator.next_u32(), 15);
    assert_eq!(generator.next_u32(), 1); // Should generate a new value
}

