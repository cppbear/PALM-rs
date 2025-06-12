// Answer 0

#[derive(Default)]
struct Core {
    generated: bool,
}

impl Core {
    fn generate(&mut self, results: &mut Vec<u64>) {
        if !self.generated {
            results.push(1);
            results.push(2);
            results.push(3);
            self.generated = true;
        }
    }
}

struct RandomGenerator {
    index: usize,
    results: Vec<u64>,
    core: Core,
    half_used: bool,
}

impl RandomGenerator {
    fn new() -> Self {
        Self {
            index: 0,
            results: vec![],
            core: Core::default(),
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
fn test_next_u64_first_call() {
    let mut rng = RandomGenerator::new();
    assert_eq!(rng.next_u64(), 1);
}

#[test]
fn test_next_u64_second_call() {
    let mut rng = RandomGenerator::new();
    rng.next_u64(); // generate the results
    assert_eq!(rng.next_u64(), 2);
}

#[test]
fn test_next_u64_third_call() {
    let mut rng = RandomGenerator::new();
    rng.next_u64(); // generate the results
    rng.next_u64(); // use second value
    assert_eq!(rng.next_u64(), 3);
}

#[test]
fn test_next_u64_fourth_call() {
    let mut rng = RandomGenerator::new();
    rng.next_u64(); // generate results
    rng.next_u64(); // use second value
    rng.next_u64(); // use third value
    assert_eq!(rng.next_u64(), 1); // should regenerate
}

