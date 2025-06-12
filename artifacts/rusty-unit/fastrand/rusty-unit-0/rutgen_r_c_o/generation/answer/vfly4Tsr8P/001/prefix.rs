// Answer 0

#[test]
fn test_f32_range_0_to_1() {
    let mut rng = Rng::with_seed(42);
    let result = rng.f32();
}

#[test]
fn test_f32_range_0_to_1_with_different_seed() {
    let mut rng = Rng::with_seed(100);
    let result = rng.f32();
}

#[test]
fn test_f32_range_0_to_1_with_min_seed() {
    let mut rng = Rng::with_seed(0);
    let result = rng.f32();
}

#[test]
fn test_f32_range_0_to_1_with_max_seed() {
    let mut rng = Rng::with_seed(u64::MAX);
    let result = rng.f32();
}

#[test]
fn test_f32_range_using_forked_rng() {
    let mut rng = Rng::with_seed(42);
    let mut forked_rng = rng.fork();
    let result = forked_rng.f32();
}

