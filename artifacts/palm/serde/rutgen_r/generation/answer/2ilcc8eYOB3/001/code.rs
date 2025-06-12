// Answer 0

#[test]
fn test_fmt_multiple_elements() {
    struct Sequence(usize);
    
    impl std::fmt::Display for Sequence {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in sequence")
            } else {
                write!(formatter, "{} elements in sequence", self.0)
            }
        }
    }

    let sequence = Sequence(5);
    let mut output = String::new();
    let result = write!(&mut output, "{}", sequence);
    assert!(result.is_ok());
    assert_eq!(output, "5 elements in sequence");
}

#[test]
fn test_fmt_large_number_of_elements() {
    struct Sequence(usize);
    
    impl std::fmt::Display for Sequence {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in sequence")
            } else {
                write!(formatter, "{} elements in sequence", self.0)
            }
        }
    }

    let sequence = Sequence(1000);
    let mut output = String::new();
    let result = write!(&mut output, "{}", sequence);
    assert!(result.is_ok());
    assert_eq!(output, "1000 elements in sequence");
}

#[test]
fn test_fmt_zero_elements() {
    struct Sequence(usize);
    
    impl std::fmt::Display for Sequence {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in sequence")
            } else {
                write!(formatter, "{} elements in sequence", self.0)
            }
        }
    }

    let sequence = Sequence(0);
    let mut output = String::new();
    let result = write!(&mut output, "{}", sequence);
    assert!(result.is_ok());
    assert_eq!(output, "0 elements in sequence");
}

