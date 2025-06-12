// Answer 0

#[test]
fn test_range_visitor_expecting() {
    use std::fmt;

    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "Expecting range");
            Ok(())
        }
    }

    let mock_formatter = &mut MockFormatter;
    let visitor = RangeVisitor::<i32> {
        expecting: "Expecting range",
        phantom: std::marker::PhantomData,
    };
    
    assert!(visitor.expecting(mock_formatter).is_ok());
}

#[test]
fn test_range_visitor_expecting_empty() {
    use std::fmt;

    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mock_formatter = &mut MockFormatter;
    let visitor = RangeVisitor::<i32> {
        expecting: "",
        phantom: std::marker::PhantomData,
    };
    
    assert!(visitor.expecting(mock_formatter).is_ok());
}

