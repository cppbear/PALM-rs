// Answer 0

#[test]
fn test_fmt_max_size_reached() {
    struct MaxSizeReachedTest {
        _priv: (),
    }

    impl fmt::Display for MaxSizeReachedTest {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("max size reached")
        }
    }

    let max_size_reached = MaxSizeReachedTest { _priv: () };
    let mut output = String::new();
    
    // Capture the write operation into a formatter
    let result = write!(&mut output, "{}", max_size_reached);
    
    // Check for no errors in writing
    assert!(result.is_ok());
    
    // Verify output is correct
    assert_eq!(output, "max size reached");
}

