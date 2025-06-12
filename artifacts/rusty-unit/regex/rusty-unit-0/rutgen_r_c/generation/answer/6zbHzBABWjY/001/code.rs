// Answer 0

#[test]
fn test_get_valid_capture() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3), None, Some(6)]);
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let result = caps.get(1);
    assert!(result.is_some());
    let m = result.unwrap();
    assert_eq!(m.text, "abc123");
    assert_eq!(m.start, 3);
    assert_eq!(m.end, 6);
}

#[test]
fn test_get_invalid_capture() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let result = caps.get(2);
    assert!(result.is_none());
}

#[test]
fn test_get_empty_capture() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3), None, None]);
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let result = caps.get(2);
    assert!(result.is_none());
}

#[test]
fn test_get_out_of_bounds_capture() {
    let text = "abc123";
    let locs = Locations(vec![Some(0), Some(3)]);
    let named_groups = Arc::new(HashMap::new());
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let result = caps.get(3);
    assert!(result.is_none());
}

