// Answer 0

#[test]
fn test_as_f64_with_integer() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        fn new_i64(value: i64) -> Self {
            TestNumber { n: N::NegInt(value) }
        }

        fn new_u64(value: u64) -> Self {
            TestNumber { n: N::PosInt(value) }
        }

        fn new_float(value: f64) -> Self {
            TestNumber { n: N::Float(value) }
        }
    }

    let int_value = Value::Number(TestNumber::new_i64(-64));
    let uint_value = Value::Number(TestNumber::new_u64(64));
    let float_value = Value::Number(TestNumber::new_float(256.0));

    assert_eq!(int_value.as_f64(), Some(-64.0));
    assert_eq!(uint_value.as_f64(), Some(64.0));
    assert_eq!(float_value.as_f64(), Some(256.0));
}

#[test]
fn test_as_f64_with_non_number() {
    let null_value = Value::Null;
    let bool_value = Value::Bool(true);
    let string_value = Value::String(String::from("not a number"));

    assert_eq!(null_value.as_f64(), None);
    assert_eq!(bool_value.as_f64(), None);
    assert_eq!(string_value.as_f64(), None);
}

#[test]
fn test_as_f64_with_number() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        fn new_float(value: f64) -> Self {
            TestNumber { n: N::Float(value) }
        }

        fn new_i64(value: i64) -> Self {
            TestNumber { n: N::NegInt(value) }
        }
    }

    let float_value = Value::Number(TestNumber::new_float(12.5));
    let negative_float_value = Value::Number(TestNumber::new_i64(-12));

    assert_eq!(float_value.as_f64(), Some(12.5));
    assert_eq!(negative_float_value.as_f64(), Some(-12.0));
}

