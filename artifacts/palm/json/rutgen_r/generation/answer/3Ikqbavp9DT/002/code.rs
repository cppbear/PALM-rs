// Answer 0

#[test]
fn test_fmt_neg_int() {
    struct TestN {
        n: N,
    }
    
    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }
    
    impl TestN {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let mut buffer = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut buffer);

    let test_instance = TestN { n: N::NegInt(-42) };
    let result = test_instance.fmt(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "-42");
}

#[test]
fn test_fmt_neg_int_zero() {
    struct TestN {
        n: N,
    }
    
    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }
    
    impl TestN {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let mut buffer = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut buffer);

    let test_instance = TestN { n: N::NegInt(0) };
    let result = test_instance.fmt(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "0");
}

#[test]
fn test_fmt_neg_int_min_value() {
    struct TestN {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    impl TestN {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let mut buffer = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut buffer);

    let test_instance = TestN { n: N::NegInt(i32::MIN) };
    let result = test_instance.fmt(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "-2147483648");
}

