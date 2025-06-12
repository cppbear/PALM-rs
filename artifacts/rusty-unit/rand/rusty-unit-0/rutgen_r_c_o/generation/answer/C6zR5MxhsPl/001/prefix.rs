// Answer 0

#[test]
fn test_step_with_minimal_state_and_increment() {
    let mut rng = Lcg64Xsh32::new(0, 1);
    rng.step();
}

#[test]
fn test_step_with_maximal_state_and_minimal_increment() {
    let mut rng = Lcg64Xsh32::new(u64::MAX, 1);
    rng.step();
}

#[test]
fn test_step_with_minimal_state_and_maximal_increment() {
    let mut rng = Lcg64Xsh32::new(0, u64::MAX);
    rng.step();
}

#[test]
fn test_step_with_arbitrary_state_and_increment() {
    let mut rng = Lcg64Xsh32::new(123456789, 987654321);
    rng.step();
}

#[test]
fn test_step_with_large_increment() {
    let mut rng = Lcg64Xsh32::new(500, 2_u64.pow(63) - 1);
    rng.step();
}

#[test]
fn test_step_with_increment_one_less_than_power_of_two() {
    let mut rng = Lcg64Xsh32::new(1, 2_u64.pow(64) - 2);
    rng.step();
}

#[test]
fn test_step_with_increment_at_power_of_two() {
    let mut rng = Lcg64Xsh32::new(1, 2_u64.pow(63));
    rng.step();
}

