// Answer 0

#[test]
fn test_helper_with_zero() {
    let result = helper((0, Some(0)));
}

#[test]
fn test_helper_with_one() {
    let result = helper((1, Some(1)));
}

#[test]
fn test_helper_with_max() {
    let result = helper((usize::MAX, Some(usize::MAX)));
}

