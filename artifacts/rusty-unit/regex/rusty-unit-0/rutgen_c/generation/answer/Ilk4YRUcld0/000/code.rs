// Answer 0

#[test]
fn test_expand_with_named_capture() {
    let named_groups = Arc::new(HashMap::from([
        ("name".to_string(), 1),
    ]));
    let locs = Locations(vec![]);
    let caps = Captures {
        text: b"test text",
        locs,
        named_groups: named_groups.clone(),
    };
    let mut dst = Vec::new();
    let replacement = b"This is a $name example.";

    // Simulating a capture for the named group
    let _ = caps.get(1); // Assuming this returns a valid Match
    caps.expand(replacement, &mut dst);

    assert_eq!(dst, b"This is a  example."); // Replace with actual Match content
}

#[test]
fn test_expand_with_number_capture() {
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(vec![]);
    let caps = Captures {
        text: b"test text",
        locs,
        named_groups,
    };
    let mut dst = Vec::new();
    let replacement = b"This is a $0 example.";

    // Simulating a capture for the full match
    let _ = caps.get(0); // Assuming this returns a valid Match
    caps.expand(replacement, &mut dst);

    assert_eq!(dst, b"This is a  example."); // Replace with actual Match content
}

#[test]
fn test_expand_with_invalid_capture() {
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(vec![]);
    let caps = Captures {
        text: b"test text",
        locs,
        named_groups,
    };
    let mut dst = Vec::new();
    let replacement = b"This is a $invalid example.";

    caps.expand(replacement, &mut dst);

    assert_eq!(dst, b"This is a  example."); // Invalid capture should result in empty string
}

#[test]
fn test_expand_with_literal_dollar() {
    let named_groups = Arc::new(HashMap::new());
    let locs = Locations(vec![]);
    let caps = Captures {
        text: b"test text",
        locs,
        named_groups,
    };
    let mut dst = Vec::new();
    let replacement = b"This is a $$ dollar sign.";

    caps.expand(replacement, &mut dst);

    assert_eq!(dst, b"This is a $ dollar sign."); // Literal $ should remain
}

