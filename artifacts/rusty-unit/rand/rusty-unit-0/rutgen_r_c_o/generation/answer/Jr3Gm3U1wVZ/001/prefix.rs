// Answer 0

#[test]
fn test_step_normal_case() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    rng.step();
}

#[test]
fn test_step_minimum_values() {
    let mut rng = Lcg128Xsl64::new(0, 1);
    rng.step();
}

#[test]
fn test_step_large_state() {
    let mut rng = Lcg128Xsl64::new(u128::MAX, 1);
    rng.step();
}

#[test]
fn test_step_large_increment() {
    let mut rng = Lcg128Xsl64::new(1, u128::MAX);
    rng.step();
}

#[test]
fn test_step_increment_one() {
    let mut rng = Lcg128Xsl64::new(1, 2);
    rng.step();
}

#[test]
fn test_step_state_increment_max() {
    let mut rng = Lcg128Xsl64::new(u128::MAX - 1, u128::MAX - 1);
    rng.step();
}

#[test]
fn test_step_state_zero_increment_max() {
    let mut rng = Lcg128Xsl64::new(0, u128::MAX);
    rng.step();
}

