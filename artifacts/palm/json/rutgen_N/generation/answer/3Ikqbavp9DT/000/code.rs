// Answer 0

#[test]
fn test_fmt_positive_integer() {
    use std::fmt;
    
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f32),
    }

    impl fmt::Display for TestNumber {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let number = TestNumber { n: N::PosInt(42) };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", number);
    assert!(result.is_ok());
    assert_eq!(buffer, "42");
}

#[test]
fn test_fmt_negative_integer() {
    use std::fmt;
    
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f32),
    }

    impl fmt::Display for TestNumber {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let number = TestNumber { n: N::NegInt(-42) };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", number);
    assert!(result.is_ok());
    assert_eq!(buffer, "-42");
}

#[test]
fn test_fmt_float() {
    use std::fmt;
    
    struct TestNumber {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f32),
    }

    impl fmt::Display for TestNumber {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let number = TestNumber { n: N::Float(3.14) };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", number);
    assert!(result.is_ok());
    assert_eq!(buffer, "3.14");
}

