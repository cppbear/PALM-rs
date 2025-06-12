// Answer 0

#[test]
fn test_bernoulli_new_valid() {
    let result_0_0 = Bernoulli::new(0.0).unwrap();
    assert_eq!(result_0_0, Bernoulli { p_int: 0 });

    let result_0_5 = Bernoulli::new(0.5).unwrap();
    assert_eq!(result_0_5, Bernoulli { p_int: (0.5 * SCALE) as u64 });

    let result_0_9999999999999999 = Bernoulli::new(0.9999999999999999).unwrap();
    assert_eq!(result_0_9999999999999999, Bernoulli { p_int: ALWAYS_TRUE });

    let result_0_1 = Bernoulli::new(1.0).unwrap();
    assert_eq!(result_0_1, Bernoulli { p_int: ALWAYS_TRUE });
}

#[test]
#[should_panic]
fn test_bernoulli_new_invalid_below_range() {
    Bernoulli::new(-0.1).unwrap();
}

#[test]
#[should_panic]
fn test_bernoulli_new_invalid_above_range() {
    Bernoulli::new(1.1).unwrap();
}

