// Answer 0

#[test]
fn test_unsigned_fmt() {
    use std::fmt;

    struct Unexpected {
        value: u32,
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.value {
                0..=0xFFFFFFFF => write!(formatter, "integer `{}`", self.value),
                _ => formatter.write_str("out of bounds"),
            }
        }
    }

    let unexpected = Unexpected { value: 42 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "integer `42`");
}

#[test]
fn test_unsigned_fmt_boundary() {
    use std::fmt;

    struct Unexpected {
        value: u32,
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.value {
                0..=0xFFFFFFFF => write!(formatter, "integer `{}`", self.value),
                _ => formatter.write_str("out of bounds"),
            }
        }
    }

    let unexpected = Unexpected { value: 0 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "integer `0`");
}

#[test]
fn test_unsigned_fmt_max() {
    use std::fmt;

    struct Unexpected {
        value: u32,
    }

    impl fmt::Display for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.value {
                0..=0xFFFFFFFF => write!(formatter, "integer `{}`", self.value),
                _ => formatter.write_str("out of bounds"),
            }
        }
    }

    let unexpected = Unexpected { value: 0xFFFFFFFF };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "integer `4294967295`");
}

