// Answer 0

#[test]
fn test_as_i128_with_negative_integer() {
    struct NegativeInteger {
        n: N,
    }
    
    impl NegativeInteger {
        fn new(value: i64) -> Self {
            NegativeInteger { n: N::NegInt(value) }
        }

        fn as_i128(&self) -> Option<i128> {
            self.n.as_i128()
        }
    }

    let neg_integer = NegativeInteger::new(-42);
    assert_eq!(neg_integer.as_i128(), Some(-42));

    let neg_integer_edge = NegativeInteger::new(i64::MIN);
    assert_eq!(neg_integer_edge.as_i128(), Some(i64::MIN as i128));
}

#[test]
fn test_as_i128_with_positive_integer() {
    struct PositiveInteger {
        n: N,
    }
    
    impl PositiveInteger {
        fn new(value: u64) -> Self {
            PositiveInteger { n: N::PosInt(value) }
        }

        fn as_i128(&self) -> Option<i128> {
            self.n.as_i128()
        }
    }

    let pos_integer = PositiveInteger::new(42);
    assert_eq!(pos_integer.as_i128(), Some(42));

    let pos_integer_edge = PositiveInteger::new(usize::MAX as u64);
    assert_eq!(pos_integer_edge.as_i128(), Some(usize::MAX as i128));
}

#[test]
fn test_as_i128_with_float() {
    struct FloatNumber {
        n: N,
    }
    
    impl FloatNumber {
        fn new(value: f64) -> Self {
            FloatNumber { n: N::Float(value) }
        }

        fn as_i128(&self) -> Option<i128> {
            self.n.as_i128()
        }
    }

    let float_number = FloatNumber::new(42.0);
    assert_eq!(float_number.as_i128(), None);
}

