// Answer 0

#[test]
fn test_as_i64_float() {
    struct FloatNumber {
        n: N,
    }

    impl From<f64> for FloatNumber {
        fn from(f: f64) -> Self {
            FloatNumber { n: N::Float(f) }
        }
    }

    let float_number = FloatNumber::from(3.14);
    assert_eq!(float_number.as_i64(), None);
}

#[test]
fn test_as_i64_float_negative() {
    struct FloatNumber {
        n: N,
    }

    impl From<f64> for FloatNumber {
        fn from(f: f64) -> Self {
            FloatNumber { n: N::Float(f) }
        }
    }

    let float_number = FloatNumber::from(-2.71);
    assert_eq!(float_number.as_i64(), None);
}

#[test]
fn test_as_i64_float_large() {
    struct FloatNumber {
        n: N,
    }

    impl From<f64> for FloatNumber {
        fn from(f: f64) -> Self {
            FloatNumber { n: N::Float(f) }
        }
    }

    let float_number = FloatNumber::from(1e20);
    assert_eq!(float_number.as_i64(), None);
}

