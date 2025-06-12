// Answer 0

#[test]
fn test_lcg64xsh32_step() {
    // Test case 1: Normal operation with small state and increment
    let mut rng1 = Lcg64Xsh32::new(1, 2);
    let initial_state1 = rng1.state;
    rng1.step();
    assert_eq!(rng1.state, initial_state1.wrapping_mul(MULTIPLIER).wrapping_add(2));

    // Test case 2: Maximum state value
    let mut rng2 = Lcg64Xsh32::new(u64::MAX, 3);
    let initial_state2 = rng2.state;
    rng2.step();
    assert_eq!(rng2.state, initial_state2.wrapping_mul(MULTIPLIER).wrapping_add(3));

    // Test case 3: Increment being 1
    let mut rng3 = Lcg64Xsh32::new(5, 1);
    let initial_state3 = rng3.state;
    rng3.step();
    assert_eq!(rng3.state, initial_state3.wrapping_mul(MULTIPLIER).wrapping_add(1));

    // Test case 4: State starts at zero
    let mut rng4 = Lcg64Xsh32::new(0, 4);
    let initial_state4 = rng4.state;
    rng4.step();
    assert_eq!(rng4.state, initial_state4.wrapping_mul(MULTIPLIER).wrapping_add(4));
}

