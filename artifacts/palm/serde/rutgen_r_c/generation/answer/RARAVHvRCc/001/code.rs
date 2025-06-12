// Answer 0

#[test]
fn test_expectation_empty_array() {
    use std::fmt::{self, Formatter};

    struct TestVisitor;

    impl TestVisitor {
        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("an empty array")
        }
    }

    let visitor = TestVisitor;
    let mut output = String::new();
    let mut formatter = Formatter::new(&mut output);

    // This should not panic and return Ok
    let result = visitor.expecting(&mut formatter);
    assert!(result.is_ok());

    // Verify the output
    assert_eq!(output, "an empty array");
}

#[test]
#[should_panic(expected = "expected string is not being written")]
fn test_expectation_panic() {
    use std::fmt::{self, Formatter};
    
    struct PanickingVisitor;

    impl PanickingVisitor {
        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            panic!("expected string is not being written")
        }
    }

    let visitor = PanickingVisitor;
    let mut formatter = Formatter::new(&mut String::new());

    // This should panic
    let _ = visitor.expecting(&mut formatter);
}

