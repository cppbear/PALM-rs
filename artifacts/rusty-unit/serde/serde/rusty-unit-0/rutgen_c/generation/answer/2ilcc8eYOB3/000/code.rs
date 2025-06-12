// Answer 0

#[test]
fn test_expected_fmt_single_element() {
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

    let value = ExpectedInSeq(1);
    let mut buffer = Vec::new();
    let result = write!(&mut buffer, "{}", value);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "1 element in sequence");
}

#[test]
fn test_expected_fmt_multiple_elements() {
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

    let value = ExpectedInSeq(5);
    let mut buffer = Vec::new();
    let result = write!(&mut buffer, "{}", value);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "5 elements in sequence");
}

