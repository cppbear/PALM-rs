// Answer 0

#[test]
fn test_name_valid_capture_group() {
    let text: &[u8] = b"Hello, World!";
    let locations = Locations(vec![]); // Assuming empty for this test
    let named_groups = Arc::new(HashMap::from([("greeting".to_string(), 0)]));

    let captures = Captures {
        text,
        locs: locations,
        named_groups: named_groups.clone(),
    };

    let result = captures.name("greeting");
    assert!(result.is_some());
    let matched = result.unwrap();
    assert_eq!(matched.text, b"Hello,");
    assert_eq!(matched.start, 0);
}

#[test]
fn test_name_invalid_capture_group() {
    let text: &[u8] = b"Hello, World!";
    let locations = Locations(vec![]); // Assuming empty for this test
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs: locations,
        named_groups,
    };

    let result = captures.name("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_name_out_of_bound_capture_group() {
    let text: &[u8] = b"Hello, World!";
    let locations = Locations(vec![]); // Assuming empty for this test
    let named_groups = Arc::new(HashMap::from([("start".to_string(), 1)]));

    let captures = Captures {
        text,
        locs: locations,
        named_groups,
    };

    let result = captures.name("start");
    assert!(result.is_none()); // For this test, we assume `self.get(1)` returns None for simplicity
}

