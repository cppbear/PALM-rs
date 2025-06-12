// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct TestStruct(f64);

    let val: TestStruct = TestStruct(f64::from_bits(0x7ff8000000000000)); // NaN representation
    assert_eq!(val.0.format_nonfinite(), "NaN");
}

