// Answer 0

#[test]
fn test_f64_lower_bound() {
    let mut rng = Rng::with_seed(0);
    let result = rng.f64();
}

#[test]
fn test_f64_upper_bound() {
    let mut rng = Rng::with_seed(u64::MAX);
    let result = rng.f64();
}

#[test]
fn test_f64_random_seed_1() {
    let mut rng = Rng::with_seed(1);
    let result = rng.f64();
}

#[test]
fn test_f64_random_seed_2() {
    let mut rng = Rng::with_seed(2);
    let result = rng.f64();
}

#[test]
fn test_f64_edge_case() {
    let mut rng = Rng::with_seed(0);
    for _ in 0..10 {
        let result = rng.f64();
    }
}

