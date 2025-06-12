// Answer 0

struct Core {
    counter: usize,
}

impl Core {
    fn generate(&mut self, results: &mut Vec<u64>) {
        results.clear();
        results.extend((0..10).map(|i| i as u64 + self.counter as u64));
        self.counter += 1;
    }
}

struct RandomGenerator {
    core: Core,
    results: Vec<u64>,
    index: usize,
    half_used: bool,
}

impl RandomGenerator {
    fn new(core: Core) -> Self {
        Self {
            core,
            results: Vec::new(),
            index: 0,
            half_used: false,
        }
    }

    fn next_u64(&mut self) -> u64 {
        if self.index >= self.results.as_ref().len() {
            self.core.generate(&mut self.results);
            self.index = 0;
        }
        
        let value = self.results.as_ref()[self.index];
        self.index += 1;
        self.half_used = false;
        value
    }
}

#[test]
fn test_next_u64_with_prep_data() {
    let core = Core { counter: 0 };
    let mut generator = RandomGenerator::new(core);

    // Generate results and get the first value
    let first_value = generator.next_u64();
    assert_eq!(first_value, 0);

    // Get the next value
    let second_value = generator.next_u64();
    assert_eq!(second_value, 1);
    
    // Continue fetching values until we reach the end of generated results
    for i in 2..10 {
        let next_value = generator.next_u64();
        assert_eq!(next_value, i);
    }

    // Now calling next_u64 should trigger regeneration of results
    let regenerated_value = generator.next_u64();
    assert_eq!(regenerated_value, 10); // First value after regeneration

    // Test some more values after regeneration
    for i in 11..20 {
        let next_value = generator.next_u64();
        assert_eq!(next_value, i);
    }
}

