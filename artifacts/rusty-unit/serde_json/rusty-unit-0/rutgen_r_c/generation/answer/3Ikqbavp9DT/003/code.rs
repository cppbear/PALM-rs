// Answer 0

#[test]
fn test_fmt_positive_integer() {
    struct Number {
        n: N,
    }
    
    #[derive(Clone, PartialEq, Eq, Hash)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    impl core::fmt::Display for Number {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let pos_int_test = Number { n: N::PosInt(42) };
    let pos_int_test_output = format!("{}", pos_int_test);
    assert_eq!(pos_int_test_output, "42");
}

#[test]
fn test_fmt_negative_integer() {
    struct Number {
        n: N,
    }
    
    #[derive(Clone, PartialEq, Eq, Hash)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    impl core::fmt::Display for Number {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let neg_int_test = Number { n: N::NegInt(-42) };
    let neg_int_test_output = format!("{}", neg_int_test);
    assert_eq!(neg_int_test_output, "-42");
}

#[test]
fn test_fmt_float() {
    struct Number {
        n: N,
    }
    
    #[derive(Clone, PartialEq, Eq, Hash)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }
    
    impl core::fmt::Display for Number {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    let float_test = Number { n: N::Float(3.14) };
    let float_test_output = format!("{}", float_test);
    assert_eq!(float_test_output, "3.14");
}

