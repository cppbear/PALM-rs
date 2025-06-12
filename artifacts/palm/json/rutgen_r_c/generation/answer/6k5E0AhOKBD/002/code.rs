// Answer 0

#[test]
fn test_is_u64_negative_integer() {
    struct NegativeInteger;
    impl Number {
        fn new(value: i64) -> Self {
            Number { n: N::NegInt(value) }
        }
    }

    let negative_number = NegativeInteger::new(-1);
    assert_eq!(negative_number.is_u64(), false);
}

