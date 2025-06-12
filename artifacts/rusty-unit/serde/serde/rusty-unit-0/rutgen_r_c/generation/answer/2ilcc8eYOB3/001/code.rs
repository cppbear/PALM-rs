// Answer 0

#[test]
fn test_expected_fmt_multiple_elements() {
    use std::fmt;

    struct ExpectedInSeq(usize);
    
    impl Expected for ExpectedInSeq {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in sequence")
            } else {
                write!(formatter, "{} elements in sequence", self.0)
            }
        }
    }

    let expected = ExpectedInSeq(5);
    let mut output = String::new();
    let result = expected.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "5 elements in sequence");
}

#[test]
fn test_expected_fmt_zero_elements() {
    use std::fmt;

    struct ExpectedInSeq(usize);
    
    impl Expected for ExpectedInSeq {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in sequence")
            } else {
                write!(formatter, "{} elements in sequence", self.0)
            }
        }
    }

    let expected = ExpectedInSeq(0);
    let mut output = String::new();
    let result = expected.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "0 elements in sequence");
}

