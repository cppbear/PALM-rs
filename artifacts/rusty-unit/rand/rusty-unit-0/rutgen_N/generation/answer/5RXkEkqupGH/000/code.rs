// Answer 0

#[derive(Debug)]
struct StepRng {
    v: u64,
    a: u64,
}

impl StepRng {
    pub fn new(initial: u64, increment: u64) -> Self {
        StepRng {
            v: initial,
            a: increment,
        }
    }

    pub fn next(&mut self) -> u64 {
        let current = self.v;
        self.v += self.a;
        current
    }
}

#[test]
fn test_step_rng_initial_value() {
    let mut rng = StepRng::new(5, 2);
    assert_eq!(rng.next(), 5);
}

#[test]
fn test_step_rng_increment() {
    let mut rng = StepRng::new(5, 2);
    rng.next(); // First call
    assert_eq!(rng.next(), 7); // 5 + 2
}

#[test]
fn test_step_rng_multiple_calls() {
    let mut rng = StepRng::new(1, 3);
    assert_eq!(rng.next(), 1);
    assert_eq!(rng.next(), 4);
    assert_eq!(rng.next(), 7);
}

#[test]
fn test_step_rng_zero_increment() {
    let mut rng = StepRng::new(10, 0);
    assert_eq!(rng.next(), 10);
    assert_eq!(rng.next(), 10);
}

#[test]
fn test_step_rng_negative_increment() {
    let mut rng = StepRng::new(10, 2);
    assert_eq!(rng.next(), 10);
    rng = StepRng::new(10, 5);
    assert_eq!(rng.next(), 10);
    assert_eq!(rng.next(), 15);
}

