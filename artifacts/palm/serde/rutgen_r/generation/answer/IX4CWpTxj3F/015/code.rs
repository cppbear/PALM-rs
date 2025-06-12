// Answer 0

#[test]
fn test_fmt_float() {
    use std::fmt::{self, Formatter};
    
    struct Unexpected {
        value: f64,
    }
    
    impl Unexpected {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            match *self {
                Unexpected::Float(f) => write!(formatter, "floating point `{}`", f),
                _ => Ok(()),
            }
        }
    }

    let float_value = Unexpected { value: 3.14 };
    let mut buffer = String::new();
    let result = float_value.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `3.14`");
}

#[test]
fn test_fmt_float_negative() {
    use std::fmt::{self, Formatter};

    struct Unexpected {
        value: f64,
    }
    
    impl Unexpected {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            match *self {
                Unexpected::Float(f) => write!(formatter, "floating point `{}`", f),
                _ => Ok(()),
            }
        }
    }

    let float_value = Unexpected { value: -2.71 };
    let mut buffer = String::new();
    let result = float_value.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `-2.71`");
}

#[test]
fn test_fmt_float_zero() {
    use std::fmt::{self, Formatter};

    struct Unexpected {
        value: f64,
    }

    impl Unexpected {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            match *self {
                Unexpected::Float(f) => write!(formatter, "floating point `{}`", f),
                _ => Ok(()),
            }
        }
    }

    let float_value = Unexpected { value: 0.0 };
    let mut buffer = String::new();
    let result = float_value.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `0`");
}

#[test]
fn test_fmt_float_infinity() {
    use std::fmt::{self, Formatter};

    struct Unexpected {
        value: f64,
    }

    impl Unexpected {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            match *self {
                Unexpected::Float(f) => write!(formatter, "floating point `{}`", f),
                _ => Ok(()),
            }
        }
    }

    let float_value = Unexpected { value: f64::INFINITY };
    let mut buffer = String::new();
    let result = float_value.fmt(&mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `inf`");
}

#[test]
fn test_fmt_float_nan() {
    use std::fmt::{self, Formatter};

    struct Unexpected {
        value: f64,
    }

    impl Unexpected {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            match *self {
                Unexpected::Float(f) => write!(formatter, "floating point `{}`", f),
                _ => Ok(()),
            }
        }
    }

    let float_value = Unexpected { value: f64::NAN };
    let mut buffer = String::new();
    let result = float_value.fmt(&mut buffer);

    assert!(result.is_ok());
    assert!(buffer.contains("floating point `NaN`"));
}

