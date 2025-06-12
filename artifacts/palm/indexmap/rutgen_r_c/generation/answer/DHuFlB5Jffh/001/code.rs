// Answer 0

#[test]
fn test_fmt_alloc_error() {
    use core::fmt::Formatter;
    use core::fmt::Result;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }
    }

    impl core::fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> Result {
            Err(core::fmt::Error)
        }
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::AllocError {
            layout: alloc::alloc::Layout::from_size_align(1, 1).unwrap(), // example layout
        },
    };

    let mut formatter = MockFormatter::new();
    let result = error.fmt(&mut formatter);
    assert!(result.is_err());
}

#[test]
fn test_fmt_capacity_overflow() {
    use core::fmt::Formatter;
    use core::fmt::Result;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }
    }

    impl core::fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> Result {
            // Simulate a successful write for this test
            assert_eq!(s, "memory allocation failed because the computed capacity exceeded the collection's maximum");
            Ok(())
        }
    }

    let error = TryReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };

    let mut formatter = MockFormatter::new();
    let result = error.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_std_error() {
    use core::fmt::Formatter;
    use core::fmt::Result;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }
    }

    impl core::fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> Result {
            // Simulate writing to formatter
            assert_eq!(s, "standard error message");
            Ok(())
        }
    }

    let std_error = alloc::collections::TryReserveError::CapacityOverflow; // Sample standard error
    let error = TryReserveError {
        kind: TryReserveErrorKind::Std(std_error),
    };

    let mut formatter = MockFormatter::new();
    let result = error.fmt(&mut formatter);
    assert!(result.is_ok());
}

