// Answer 0

#[derive(Debug)]
struct Bernoulli {
    p_int: u64,
}

trait Rng {
    fn random(&mut self) -> u64;
}

struct MockRng {
    value: u64,
}

impl Rng for MockRng {
    fn random(&mut self) -> u64 {
        self.value
    }
}

const ALWAYS_TRUE: u64 = 1;

#[test]
fn test_sample_returns_false_when_p_int_is_less_than_random() {
    let mut rng = MockRng { value: 5 };
    let bernoulli = Bernoulli { p_int: 10 };

    assert_eq!(bernoulli.sample(&mut rng), true);
}

#[test]
fn test_sample_returns_true_when_p_int_is_equal_to_random() {
    let mut rng = MockRng { value: 10 };
    let bernoulli = Bernoulli { p_int: 10 };

    assert_eq!(bernoulli.sample(&mut rng), false);
}

#[test]
fn test_sample_returns_false_when_p_int_is_greater_than_random() {
    let mut rng = MockRng { value: 1 };
    let bernoulli = Bernoulli { p_int: 10 };
    
    assert_eq!(bernoulli.sample(&mut rng), true);
}

#[test]
fn test_sample_panic_if_p_int_is_always_true() {
    let mut rng = MockRng { value: 0 };
    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };

    assert_eq!(bernoulli.sample(&mut rng), true);
}

