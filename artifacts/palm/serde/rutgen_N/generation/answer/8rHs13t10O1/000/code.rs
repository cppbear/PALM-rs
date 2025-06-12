// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("option")
        }
    }

    let test_struct = TestStruct;
    let mut formatter = TestFormatter { output: String::new() };

    let result = test_struct.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.output, "option");
}

