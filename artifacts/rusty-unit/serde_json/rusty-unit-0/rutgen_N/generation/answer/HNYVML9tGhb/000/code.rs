// Answer 0

#[test]
fn test_is_u64_with_valid_u64() {
    struct MockNumber {
        value: Option<u64>,
    }

    impl MockNumber {
        fn is_u64(&self) -> bool {
            self.value.is_some() && self.value.unwrap() <= u64::MAX
        }
    }

    enum Value {
        Number(MockNumber),
    }

    let valid_value = Value::Number(MockNumber { value: Some(64) });
    
    assert!(is_u64(&valid_value));
}

#[test]
fn test_is_u64_with_negative_integer() {
    struct MockNumber {
        value: Option<i64>,
    }

    impl MockNumber {
        fn is_u64(&self) -> bool {
            self.value.map_or(false, |v| v >= 0 && v <= u64::MAX as i64)
        }
    }

    enum Value {
        Number(MockNumber),
    }

    let negative_value = Value::Number(MockNumber { value: Some(-64) });
    
    assert!(!is_u64(&negative_value));
}

#[test]
fn test_is_u64_with_decimal_value() {
    struct MockNumber {
        is_integer: bool,
    }

    impl MockNumber {
        fn is_u64(&self) -> bool {
            self.is_integer
        }
    }

    enum Value {
        Number(MockNumber),
    }

    let decimal_value = Value::Number(MockNumber { is_integer: false });
    
    assert!(!is_u64(&decimal_value));
}

