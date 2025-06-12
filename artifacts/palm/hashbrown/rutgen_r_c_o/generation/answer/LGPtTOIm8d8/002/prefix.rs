// Answer 0

#[test]
fn test_capacity_overflow_fallible() {
    let fallibility = Fallibility::Fallible;
    let result = fallibility.capacity_overflow();
}

#[test]
#[should_panic]
fn test_capacity_overflow_infallible() {
    let fallibility = Fallibility::Infallible;
    let _ = fallibility.capacity_overflow();
}

