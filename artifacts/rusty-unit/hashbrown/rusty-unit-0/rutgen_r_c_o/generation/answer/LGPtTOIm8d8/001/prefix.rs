// Answer 0

#[test]
fn test_capacity_overflow_infallible() {
    let fallibility = Fallibility::Infallible;
    let _result = fallibility.capacity_overflow();
}

