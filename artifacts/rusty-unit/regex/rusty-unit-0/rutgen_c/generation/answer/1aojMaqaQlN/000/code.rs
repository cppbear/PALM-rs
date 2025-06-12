// Answer 0

#[test]
fn test_name_valid_capture_group() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let text = "Hello, world!";
    let locs = Locations(vec![]); // Initialize with empty locations for simplicity
    let named_groups = Arc::new(HashMap::from([
        ("greet".to_string(), 0),
    ]));
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    
    let result = captures.name("greet");
    assert!(result.is_some());
    let matched = result.unwrap();
    assert_eq!(matched.text, "Hello, world!");
    assert_eq!(matched.start, 0);
    assert_eq!(matched.end, 13);
}

#[test]
fn test_name_invalid_capture_group() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let text = "Hello, world!";
    let locs = Locations(vec![]); // Initialize with empty locations for simplicity
    let named_groups = Arc::new(HashMap::from([
        ("greet".to_string(), 0),
    ]));
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    
    let result = captures.name("farewell");
    assert!(result.is_none());
}

#[test]
fn test_name_no_capture_groups() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let text = "Hello, world!";
    let locs = Locations(vec![]); // Initialize with empty locations for simplicity
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    
    let result = captures.name("greet");
    assert!(result.is_none());
}

