// Answer 0

#[test]
fn test_as_f64_with_f64_number() {
    struct Value {
        number: Option<f64>,
    }

    impl Value {
        fn as_f64(&self) -> Option<f64> {
            match &self.number {
                Some(n) => Some(*n),
                None => None,
            }
        }
    }

    let v = Value { number: Some(256.0) };
    assert_eq!(v.as_f64(), Some(256.0));
}

#[test]
fn test_as_f64_with_i64_number() {
    struct Value {
        number: Option<i64>,
    }

    impl Value {
        fn as_f64(&self) -> Option<f64> {
            match &self.number {
                Some(n) => Some(*n as f64),
                None => None,
            }
        }
    }

    let v = Value { number: Some(64) };
    assert_eq!(v.as_f64(), Some(64.0));
}

#[test]
fn test_as_f64_with_negative_i64_number() {
    struct Value {
        number: Option<i64>,
    }

    impl Value {
        fn as_f64(&self) -> Option<f64> {
            match &self.number {
                Some(n) => Some(*n as f64),
                None => None,
            }
        }
    }

    let v = Value { number: Some(-64) };
    assert_eq!(v.as_f64(), Some(-64.0));
}

#[test]
fn test_as_f64_with_none() {
    struct Value {
        number: Option<i64>,
    }

    impl Value {
        fn as_f64(&self) -> Option<f64> {
            match &self.number {
                Some(n) => Some(*n as f64),
                None => None,
            }
        }
    }

    let v = Value { number: None };
    assert_eq!(v.as_f64(), None);
}

