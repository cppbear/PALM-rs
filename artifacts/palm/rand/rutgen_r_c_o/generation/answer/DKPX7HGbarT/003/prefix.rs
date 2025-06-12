// Answer 0

#[test]
fn test_bernoulli_new_invalid_probability_negative() {
    let result = Bernoulli::new(-1.0);
}

#[test]
fn test_bernoulli_new_invalid_probability_above_one() {
    let result = Bernoulli::new(1.1);
}

