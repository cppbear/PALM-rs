// Answer 0

#[test]
fn test_fmt_debug() {
    use std::fmt;
    
    struct DebugWrapper(i32);

    impl fmt::Debug for DebugWrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    let value = DebugWrapper(42);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    
    assert!(result.is_ok());
    assert_eq!(output, "42");
}

#[test]
fn test_fmt_debug_empty() {
    use std::fmt;

    struct DebugWrapper(i32);

    impl fmt::Debug for DebugWrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    let value = DebugWrapper(0);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", value);
    
    assert!(result.is_ok());
    assert_eq!(output, "0");
}

