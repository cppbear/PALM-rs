// Answer 0

#[test]
fn test_bernoulli_new_valid_probability() {
    let result = Bernoulli::new(1.0);
}

#[test]
#[should_panic]
fn test_bernoulli_new_probability_greater_than_one() {
    let result = Bernoulli::new(1.1);
}

#[test]
#[should_panic]
fn test_bernoulli_new_probability_less_than_zero() {
    let result = Bernoulli::new(-0.1);
}

