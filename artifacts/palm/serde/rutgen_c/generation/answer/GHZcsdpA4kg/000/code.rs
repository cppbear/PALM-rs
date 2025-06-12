// Answer 0

#[test]
fn test_collect_str_with_string() {
    struct MockSerializer<'a>(&'a mut String);

    impl<'a> std::fmt::Write for MockSerializer<'a> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = String::new();
    let serializer = MockSerializer(&mut output);
    let result = serializer.collect_str(&"Hello, World!");

    assert!(result.is_ok());
    assert_eq!(output, "Hello, World!");
}

#[test]
fn test_collect_str_with_char() {
    struct MockSerializer<'a>(&'a mut String);

    impl<'a> std::fmt::Write for MockSerializer<'a> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = String::new();
    let serializer = MockSerializer(&mut output);
    let result = serializer.collect_str(&'A');

    assert!(result.is_ok());
    assert_eq!(output, "A");
}

#[test]
fn test_collect_str_with_empty_string() {
    struct MockSerializer<'a>(&'a mut String);

    impl<'a> std::fmt::Write for MockSerializer<'a> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = String::new();
    let serializer = MockSerializer(&mut output);
    let result = serializer.collect_str(&"");

    assert!(result.is_ok());
    assert_eq!(output, "");
}

