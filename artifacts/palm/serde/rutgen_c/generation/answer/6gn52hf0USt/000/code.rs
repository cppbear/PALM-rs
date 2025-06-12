// Answer 0

#[test]
fn test_expectation_formatting() {
    use std::fmt::{self, Formatter};

    struct MockFormatter<'a> {
        buffer: &'a mut String,
    }

    impl<'a> Formatter<'a> for MockFormatter<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
        // Implement any other required methods...
    }

    let enum_name = "TestEnum";
    let mut buffer = String::new();
    let visitor = AdjacentlyTaggedEnumVariantVisitor {
        enum_name,
        fields_enum: PhantomData,
    };

    let result = visitor.expecting(&mut MockFormatter { buffer: &mut buffer });

    assert!(result.is_ok());
    assert_eq!(buffer, "variant of enum TestEnum");
}

#[test]
fn test_expectation_formatting_empty_enum_name() {
    use std::fmt::{self, Formatter};

    struct MockFormatter<'a> {
        buffer: &'a mut String,
    }

    impl<'a> Formatter<'a> for MockFormatter<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
        // Implement any other required methods...
    }

    let enum_name = "";
    let mut buffer = String::new();
    let visitor = AdjacentlyTaggedEnumVariantVisitor {
        enum_name,
        fields_enum: PhantomData,
    };

    let result = visitor.expecting(&mut MockFormatter { buffer: &mut buffer });

    assert!(result.is_ok());
    assert_eq!(buffer, "variant of enum ");
}

