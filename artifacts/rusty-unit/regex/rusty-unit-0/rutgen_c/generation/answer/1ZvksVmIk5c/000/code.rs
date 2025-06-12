// Answer 0

#[test]
fn test_captures_debug_fmt() {
    // Prepare dummy data for testing
    let text: &[u8] = b"test";
    let locs = Locations(vec![]); // Assumed to create an empty Locations
    let named_groups = Arc::new(HashMap::new());

    // Create an instance of Captures
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    // Prepare a buffer for formatting
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", captures);

    // Assert the result is Ok
    assert!(result.is_ok());
    assert!(buffer.contains("Captures"));
}

#[test]
fn test_captures_debug_empty() {
    // Prepare dummy data for testing
    let text: &[u8] = b"";
    let locs = Locations(vec![]); // Assumed to create an empty Locations
    let named_groups = Arc::new(HashMap::new());

    // Create an instance of Captures
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    // Prepare a buffer for formatting
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", captures);

    // Assert the result is Ok
    assert!(result.is_ok());
    assert!(buffer.contains("Captures"));
}

