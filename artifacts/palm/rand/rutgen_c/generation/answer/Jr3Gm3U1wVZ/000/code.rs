// Answer 0

#[test]
fn test_lcg_step() {
    // Arrange
    let mut rng = Lcg128Xsl64::new(1, 2);
    let expected_state = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645.wrapping_mul(1).wrapping_add(3); // MULTIPLIER * state + increment

    // Act
    rng.step();

    // Assert
    assert_eq!(rng.state, expected_state);
}

#[test]
fn test_lcg_step_with_large_values() {
    // Arrange
    let mut rng = Lcg128Xsl64::new(u128::MAX, 2);
    let expected_state = MULTIPLIER.wrapping_mul(u128::MAX).wrapping_add(3); // MULTIPLIER * state + increment

    // Act
    rng.step();

    // Assert
    assert_eq!(rng.state, expected_state);
}

#[test]
fn test_lcg_step_with_zero_state_and_increment() {
    // Arrange
    let mut rng = Lcg128Xsl64::new(0, 0);
    let expected_state = MULTIPLIER.wrapping_mul(0).wrapping_add(1); // MULTIPLIER * state + increment with increment being 1

    // Act
    rng.step();

    // Assert
    assert_eq!(rng.state, expected_state);
}

