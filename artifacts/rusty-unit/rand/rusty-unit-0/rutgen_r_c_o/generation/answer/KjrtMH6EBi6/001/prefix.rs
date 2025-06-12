// Answer 0

#[test]
fn test_step_with_min_state_and_min_increment() {
    let mut rng = Lcg128CmDxsm64::new(0, 1);
    rng.step();
}

#[test]
fn test_step_with_min_state_and_max_increment() {
    let mut rng = Lcg128CmDxsm64::new(0, u128::MAX);
    rng.step();
}

#[test]
fn test_step_with_max_state_and_min_increment() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX, 1);
    rng.step();
}

#[test]
fn test_step_with_max_state_and_max_increment() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX, u128::MAX);
    rng.step();
}

#[test]
fn test_step_with_mid_state_and_min_increment() {
    let mid_state = u128::MAX / 2;
    let mut rng = Lcg128CmDxsm64::new(mid_state, 1);
    rng.step();
}

#[test]
fn test_step_with_mid_state_and_max_increment() {
    let mid_state = u128::MAX / 2;
    let mut rng = Lcg128CmDxsm64::new(mid_state, u128::MAX);
    rng.step();
}

#[test]
fn test_step_with_random_state_and_increment() {
    let mut rng = Lcg128CmDxsm64::new(12345678901234567890, 98765432101234567890);
    rng.step();
}

