// Answer 0

#[test]
fn test_digit_zero_base() {
    let base = 0;
    digit(base);
}

#[test]
fn test_digit_one_base() {
    let base = 1;
    digit(base);
}

#[test]
fn test_digit_thirty_six_base() {
    let base = 36;
    digit(base);
}

#[test]
#[should_panic]
fn test_digit_negative_base() {
    let base = u32::MAX; // This should trigger panic as it's equivalent to -1 in unsigned representation.
    digit(base);
}

