// Answer 0

#[test]
fn test_captures_debug_empty() {
    let text = "";
    let locs = Locations(Vec::new());
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    let captures_debug = CapturesDebug(&captures);
    let result = format!("{:?}", captures_debug);
    assert_eq!(result, "{}");
}

#[test]
fn test_captures_debug_single_group() {
    let text = "Hello, World!";
    let locs = Locations(vec![Some((0, 5))]); // "Hello" captures from 0 to 5
    let mut named_groups = HashMap::new();
    named_groups.insert("greeting".to_string(), 0);
    let named_groups = Arc::new(named_groups);
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    let captures_debug = CapturesDebug(&captures);
    let result = format!("{:?}", captures_debug);
    assert!(result.contains("greeting"));
    assert!(result.contains("\"Hello\""));
}

#[test]
fn test_captures_debug_multiple_groups() {
    let text = "Hello, World!";
    let locs = Locations(vec![Some((0, 5)), Some((7, 12))]); // "Hello" and "World" captures
    let mut named_groups = HashMap::new();
    named_groups.insert("greeting".to_string(), 0);
    named_groups.insert("target".to_string(), 1);
    let named_groups = Arc::new(named_groups);
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    let captures_debug = CapturesDebug(&captures);
    let result = format!("{:?}", captures_debug);
    assert!(result.contains("greeting"));
    assert!(result.contains("\"Hello\""));
    assert!(result.contains("target"));
    assert!(result.contains("\"World\""));
}

