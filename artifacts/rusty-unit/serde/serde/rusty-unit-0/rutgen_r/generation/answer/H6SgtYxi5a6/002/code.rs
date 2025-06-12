// Answer 0

#[test]
fn test_fmt_one_element() {
    use std::fmt;

    struct TestStruct(usize);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in map")
            } else {
                write!(formatter, "{} elements in map", self.0)
            }
        }
    }

    let one_element = TestStruct(1);
    let mut output = String::new();
    let result = write!(&mut output, "{}", one_element);
    
    assert!(result.is_ok());
    assert_eq!(output, "1 element in map");
}

#[test]
fn test_fmt_multiple_elements() {
    use std::fmt;

    struct TestStruct(usize);

    impl fmt::Display for TestStruct {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            if self.0 == 1 {
                formatter.write_str("1 element in map")
            } else {
                write!(formatter, "{} elements in map", self.0)
            }
        }
    }

    let multiple_elements = TestStruct(5);
    let mut output = String::new();
    let result = write!(&mut output, "{}", multiple_elements);
    
    assert!(result.is_ok());
    assert_eq!(output, "5 elements in map");
}

