// Answer 0

#[test]
fn test_expecting_with_valid_formatter() {
    use std::fmt::Formatter;
    
    struct TestFormatter {
        output: String,
    }

    impl Formatter {
        fn new() -> Self {
            TestFormatter {
                output: String::new(),
            }
        }
    }

    let visitor = InternallyTaggedUnitVisitor {
        type_name: "MyType",
        variant_name: "MyVariant",
    };
    
    let mut formatter = TestFormatter::new();
    
    let result = visitor.expecting(&mut formatter);
    
    assert_eq!(result, Ok(()));
    assert_eq!(formatter.output, "unit variant MyType::MyVariant");
}

