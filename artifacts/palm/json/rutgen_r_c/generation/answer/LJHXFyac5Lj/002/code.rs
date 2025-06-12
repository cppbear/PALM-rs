// Answer 0

#[test]
fn test_as_i64_negint() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        pub fn as_i64(&self) -> Option<i64> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => {
                    if n <= i64::MAX as u64 {
                        Some(n as i64)
                    } else {
                        None
                    }
                }
                N::NegInt(n) => Some(n),
                N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let negative_number = TestNumber { n: N::NegInt(-42) };
    assert_eq!(negative_number.as_i64(), Some(-42));

    let large_negative_number = TestNumber { n: N::NegInt(i64::MIN) };
    assert_eq!(large_negative_number.as_i64(), Some(i64::MIN));

    let positive_number = TestNumber { n: N::PosInt(0) };
    assert_eq!(positive_number.as_i64(), Some(0));

    let large_positive_number = TestNumber { n: N::PosInt(u64::MAX) };
    assert_eq!(large_positive_number.as_i64(), None);
}

#[test]
fn test_as_i64_float() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        pub fn as_i64(&self) -> Option<i64> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => {
                    if n <= i64::MAX as u64 {
                        Some(n as i64)
                    } else {
                        None
                    }
                }
                N::NegInt(n) => Some(n),
                N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let float_number = TestNumber { n: N::Float(3.14) };
    assert_eq!(float_number.as_i64(), None);
}

