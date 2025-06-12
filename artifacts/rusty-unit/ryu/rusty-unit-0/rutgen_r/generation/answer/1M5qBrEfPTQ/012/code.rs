// Answer 0

#[test]
fn test_decimal_length17() {
    let v: u64 = 100000; // This value respects the constraints and is expected to yield the correct output.
    assert_eq!(decimal_length17(v), 6);
}

