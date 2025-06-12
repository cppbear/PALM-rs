// Answer 0

#[test]
fn test_fmt_multiple_elements() {
    struct ElementCount(u32);

    impl std::fmt::Display for ElementCount {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in map")
            } else {
                write!(formatter, "{} elements in map", self.0)
            }
        }
    }

    let count = ElementCount(5);
    let mut output = String::new();
    let result = write!(&mut output, "{}", count);
    
    assert!(result.is_ok());
    assert_eq!(output, "5 elements in map");
}

#[test]
fn test_fmt_large_element_count() {
    struct ElementCount(u32);

    impl std::fmt::Display for ElementCount {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in map")
            } else {
                write!(formatter, "{} elements in map", self.0)
            }
        }
    }

    let count = ElementCount(100);
    let mut output = String::new();
    let result = write!(&mut output, "{}", count);
    
    assert!(result.is_ok());
    assert_eq!(output, "100 elements in map");
}

#[test]
fn test_fmt_zero_element_count() {
    struct ElementCount(u32);

    impl std::fmt::Display for ElementCount {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in map")
            } else {
                write!(formatter, "{} elements in map", self.0)
            }
        }
    }

    let count = ElementCount(0);
    let mut output = String::new();
    let result = write!(&mut output, "{}", count);
    
    assert!(result.is_ok());
    assert_eq!(output, "0 elements in map");
}

