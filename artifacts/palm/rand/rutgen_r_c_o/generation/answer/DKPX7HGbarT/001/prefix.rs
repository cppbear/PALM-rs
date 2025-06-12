// Answer 0

#[test]
fn test_bernoulli_new_zero() {
    let result = Bernoulli::new(0.0);
}

#[test]
fn test_bernoulli_new_small_positive() {
    let result = Bernoulli::new(0.0000000000000001);
}

#[test]
fn test_bernoulli_new_large_positive() {
    let result = Bernoulli::new(0.9999999999999999);
}

#[test]
fn test_bernoulli_new_one() {
    let result = Bernoulli::new(1.0);
}

#[test]
#[should_panic]
fn test_bernoulli_new_negative() {
    let result = Bernoulli::new(-0.1);
}

#[test]
#[should_panic]
fn test_bernoulli_new_above_one() {
    let result = Bernoulli::new(1.1);
}

