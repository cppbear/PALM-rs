// Answer 0

#[test]
fn test_as_f64_negative_integer() {
    struct NumberTest {
        n: N,
    }

    impl NumberTest {
        pub fn as_f64(&self) -> Option<f64> {
            match self.n {
                N::PosInt(n) => Some(n as f64),
                N::NegInt(n) => Some(n as f64),
                N::Float(n) => Some(n),
            }
        }
    }

    let negative_int = NumberTest { n: N::NegInt(-42) };
    assert_eq!(negative_int.as_f64(), Some(-42.0));
}

#[test]
fn test_as_f64_positive_integer() {
    struct NumberTest {
        n: N,
    }

    impl NumberTest {
        pub fn as_f64(&self) -> Option<f64> {
            match self.n {
                N::PosInt(n) => Some(n as f64),
                N::NegInt(n) => Some(n as f64),
                N::Float(n) => Some(n),
            }
        }
    }

    let positive_int = NumberTest { n: N::PosInt(42) };
    assert_eq!(positive_int.as_f64(), Some(42.0));
}

#[test]
fn test_as_f64_float() {
    struct NumberTest {
        n: N,
    }

    impl NumberTest {
        pub fn as_f64(&self) -> Option<f64> {
            match self.n {
                N::PosInt(n) => Some(n as f64),
                N::NegInt(n) => Some(n as f64),
                N::Float(n) => Some(n),
            }
        }
    }

    let float_value = NumberTest { n: N::Float(3.14) };
    assert_eq!(float_value.as_f64(), Some(3.14));
}

