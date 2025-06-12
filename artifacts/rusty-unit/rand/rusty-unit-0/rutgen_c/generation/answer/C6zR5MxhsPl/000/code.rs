// Answer 0

#[test]
fn test_step_function_increments_state() {
    let mut rng = Lcg64Xsh32::new(0, 0);
    let initial_state = rng.state;

    rng.step();
    assert!(rng.state > initial_state, "State should increment after step");
}

#[test]
fn test_step_function_with_large_initial_state() {
    let mut rng = Lcg64Xsh32::new(u64::MAX, 0);
    let initial_state = rng.state;

    rng.step();
    assert!(rng.state > initial_state, "State should wrap around and increment after step");
}

#[test]
fn test_step_function_with_increment() {
    let mut rng = Lcg64Xsh32::new(12345, 67890);
    let initial_state = rng.state;

    rng.step();
    assert!(rng.state > initial_state, "State should increment after step");
}

#[test]
fn test_step_function_with_different_streams() {
    let mut rng1 = Lcg64Xsh32::new(1, 1);
    let mut rng2 = Lcg64Xsh32::new(1, 2);
    
    rng1.step();
    rng2.step();
    
    assert!(rng1.state != rng2.state, "States should differ with different streams");
}

