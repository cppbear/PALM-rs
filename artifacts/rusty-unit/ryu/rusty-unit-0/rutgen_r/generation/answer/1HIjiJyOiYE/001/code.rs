// Answer 0

#[test]
fn test_format_nan() {
    struct TestFloat;

    impl Float for TestFloat {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "NaN"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(TestFloat);
    assert_eq!(result, "NaN");
}

#[test]
fn test_format_positive_infinity() {
    struct TestFloat;

    impl Float for TestFloat {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "inf"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(TestFloat);
    assert_eq!(result, "inf");
}

#[test]
fn test_format_negative_infinity() {
    struct TestFloat;

    impl Float for TestFloat {
        fn is_nonfinite(&self) -> bool {
            true
        }

        fn format_nonfinite(&self) -> &str {
            "-inf"
        }
    }

    let mut buffer = String::new();
    let result = buffer.format(TestFloat);
    assert_eq!(result, "-inf");
}

