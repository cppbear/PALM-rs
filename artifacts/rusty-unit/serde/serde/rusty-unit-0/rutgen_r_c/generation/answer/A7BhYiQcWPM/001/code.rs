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

    let mut formatter = TestFormatter { output: String::new() };
    let visitor = TagOrContentFieldVisitor {
        tag: "tag_field",
        content: "content_field",
    };

    assert!(visitor.expecting(&mut formatter).is_ok());
    assert_eq!(formatter.output, "\"tag_field\" or \"content_field\"");
}

#[test]
fn test_expectation_empty_fields() {
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let visitor = TagOrContentFieldVisitor {
        tag: "",
        content: "",
    };

    assert!(visitor.expecting(&mut formatter).is_ok());
    assert_eq!(formatter.output, "\"\" or \"\"");
}

#[test]
fn test_expectation_special_characters() {
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let visitor = TagOrContentFieldVisitor {
        tag: "tag_with_special_chars!@#",
        content: "content_with_special_chars$%^",
    };

    assert!(visitor.expecting(&mut formatter).is_ok());
    assert_eq!(formatter.output, "\"tag_with_special_chars!@#\" or \"content_with_special_chars$%^\"");
}

