// Answer 0

#[test]
fn test_serialize_unit_variant() {
    use std::fmt::{self, Display, Formatter};

    struct MockSerializer<'a>(&'a mut String);

    impl<'a> fmt::Write for MockSerializer<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    let mut output = String::new();
    let mut serializer = MockSerializer(&mut output);

    let result = serializer.serialize_unit_variant("TestName", 0, "TestVariant");
    assert!(result.is_ok());
    assert_eq!(output, "TestVariant");
}

