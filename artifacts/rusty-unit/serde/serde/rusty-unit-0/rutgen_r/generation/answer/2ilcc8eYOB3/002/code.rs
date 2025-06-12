// Answer 0

#[test]
fn test_fmt_single_element() {
    struct TestStruct(u32);

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                f.write_str("1 element in sequence")
            } else {
                write!(f, "{} elements in sequence", self.0)
            }
        }
    }

    let test_instance = TestStruct(1);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);

    assert!(result.is_ok());
    assert_eq!(output, "1 element in sequence");
}

#[test]
fn test_fmt_multiple_elements() {
    struct TestStruct(u32);

    impl std::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.0 == 1 {
                f.write_str("1 element in sequence")
            } else {
                write!(f, "{} elements in sequence", self.0)
            }
        }
    }

    let test_instance = TestStruct(5);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", test_instance);

    assert!(result.is_ok());
    assert_eq!(output, "5 elements in sequence");
}

