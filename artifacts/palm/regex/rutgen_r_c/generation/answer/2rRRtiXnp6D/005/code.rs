// Answer 0

#[test]
fn test_expand_bytes_with_named_capture() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let named_groups = Arc::new(HashMap::from([
        (String::from("group1"), 0),
    ]));

    let captures = re_bytes::Captures {
        text: b"Hello World",
        locs: Locations::new(vec![(0, 5)]), // Match for 'Hello'
        named_groups,
    };

    let mut dst = Vec::new();
    let replacement = b"Greetings from ${group1}!";

    expand_bytes(&captures, replacement, &mut dst);

    assert_eq!(dst, b"Greetings from Hello!");
}

#[test]
fn test_expand_bytes_with_number_capture() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let named_groups = Arc::new(HashMap::new()); // No named groups

    let captures = re_bytes::Captures {
        text: b"Hello World",
        locs: Locations::new(vec![(0, 5)]), // Match for 'Hello'
        named_groups,
    };

    let mut dst = Vec::new();
    let replacement = b"Say: $0!"; // Reference the first capture by number

    expand_bytes(&captures, replacement, &mut dst);

    assert_eq!(dst, b"Say: Hello!"); // Should expand to 'Say: Hello!'
}

#[test]
fn test_expand_bytes_with_empty_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let named_groups = Arc::new(HashMap::new()); // No named groups

    let captures = re_bytes::Captures {
        text: b"Hello World",
        locs: Locations::new(vec![(0, 5)]), // Match for 'Hello'
        named_groups,
    };

    let mut dst = Vec::new();
    let replacement = b""; // Empty replacement

    expand_bytes(&captures, replacement, &mut dst);

    assert_eq!(dst, b""); // No replacements should result in an empty output
}

#[test]
fn test_expand_bytes_with_invalid_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let named_groups = Arc::new(HashMap::new()); // No named groups

    let captures = re_bytes::Captures {
        text: b"Hello World",
        locs: Locations::new(vec![(0, 5)]), // Match for 'Hello'
        named_groups,
    };

    let mut dst = Vec::new();
    let replacement = b"Text with invalid $capture name"; // Should not panic

    expand_bytes(&captures, replacement, &mut dst);

    assert_eq!(dst, b"Text with invalid $capture name"); // Should remain unchanged
}

