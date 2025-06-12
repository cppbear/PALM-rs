// Answer 0

#[test]
fn test_name_valid_capture_group() {
    let text: &[u8] = b"Hello, world!";
    let locs = Locations(vec![]); // Placeholder for actual Locations structure
    let named_groups = Arc::new(HashMap::from([
        (String::from("greeting"), 0),
    ]));
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let result = captures.name("greeting");
    assert_eq!(result, Some(Match { text, start: 0, end: 5 }));
}

#[test]
fn test_name_invalid_capture_group() {
    let text: &[u8] = b"Hello, world!";
    let locs = Locations(vec![]); // Placeholder for actual Locations structure
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let result = captures.name("nonexistent");
    assert_eq!(result, None);
}

#[test]
fn test_name_empty_capture_group_name() {
    let text: &[u8] = b"Hello, world!";
    let locs = Locations(vec![]); // Placeholder for actual Locations structure
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let result = captures.name("");
    assert_eq!(result, None);
}

#[test]
fn test_name_capture_group_with_multiple_entries() {
    let text: &[u8] = b"Hello, world!";
    let locs = Locations(vec![]); // Placeholder for actual Locations structure
    let named_groups = Arc::new(HashMap::from([
        (String::from("greeting"), 0),
        (String::from("planet"), 7),
    ]));

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let result_greeting = captures.name("greeting");
    assert_eq!(result_greeting, Some(Match { text, start: 0, end: 5 }));

    let result_planet = captures.name("planet");
    assert_eq!(result_planet, Some(Match { text, start: 7, end: 12 }));
}

