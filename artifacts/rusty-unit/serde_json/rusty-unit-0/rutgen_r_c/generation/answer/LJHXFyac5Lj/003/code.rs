// Answer 0

#[test]
fn test_as_i64_positive_integer_max() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        pub fn new_pos_int(value: u64) -> Self {
            TestNumber { n: N::PosInt(value) }
        }

        pub fn as_i64(&self) -> Option<i64> {
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
        }
    }

    let number = TestNumber::new_pos_int(i64::MAX as u64);
    assert_eq!(number.as_i64(), Some(i64::MAX));
}

#[test]
fn test_as_i64_positive_integer_below_max() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        pub fn new_pos_int(value: u64) -> Self {
            TestNumber { n: N::PosInt(value) }
        }

        pub fn as_i64(&self) -> Option<i64> {
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
        }
    }

    let number = TestNumber::new_pos_int(42);
    assert_eq!(number.as_i64(), Some(42));
}

#[test]
fn test_as_i64_negative_integer() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        pub fn new_neg_int(value: i64) -> Self {
            TestNumber { n: N::NegInt(value) }
        }

        pub fn as_i64(&self) -> Option<i64> {
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
        }
    }

    let number = TestNumber::new_neg_int(-10);
    assert_eq!(number.as_i64(), Some(-10));
}

#[test]
fn test_as_i64_float_value() {
    struct TestNumber {
        n: N,
    }

    impl TestNumber {
        pub fn new_float(value: f64) -> Self {
            TestNumber { n: N::Float(value) }
        }

        pub fn as_i64(&self) -> Option<i64> {
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
        }
    }

    let number = TestNumber::new_float(3.14);
    assert_eq!(number.as_i64(), None);
}

