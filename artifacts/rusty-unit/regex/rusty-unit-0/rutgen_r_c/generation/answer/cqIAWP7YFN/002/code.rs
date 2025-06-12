// Answer 0

fn test_captures_debug_fmt_named_group() {
    struct TestCaptures {
        locs: Locations,
        text: &'static [u8],
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![Slot(0, 3), Slot(4, 7)]);
    let text: &'static [u8] = b"abc def";
    let named_groups = Arc::new(HashMap::from([
        ("group1".to_string(), 0),
        ("group2".to_string(), 1),
    ]));

    let captures = TestCaptures {
        locs,
        text,
        named_groups,
    };

    let captures_debug = CapturesDebug(&captures);
    let result = format!("{:?}", captures_debug);
    assert!(result.contains("group1"));
    assert!(result.contains("group2"));
    assert!(result.contains("abc"));
    assert!(result.contains("def"));
}

fn test_captures_debug_fmt_no_named_group() {
    struct TestCaptures {
        locs: Locations,
        text: &'static [u8],
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![Slot(0, 3), Slot(4, 7)]);
    let text: &'static [u8] = b"xyz abc";
    let named_groups = Arc::new(HashMap::new());

    let captures = TestCaptures {
        locs,
        text,
        named_groups,
    };

    let captures_debug = CapturesDebug(&captures);
    let result = format!("{:?}", captures_debug);
    assert!(result.contains("0"));
    assert!(result.contains("xyz"));
    assert!(result.contains("abc"));
}

fn test_captures_debug_fmt_empty_locations() {
    struct TestCaptures {
        locs: Locations,
        text: &'static [u8],
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![]);
    let text: &'static [u8] = b"";
    let named_groups = Arc::new(HashMap::new());

    let captures = TestCaptures {
        locs,
        text,
        named_groups,
    };

    let captures_debug = CapturesDebug(&captures);
    let result = format!("{:?}", captures_debug);
    assert!(result.is_empty());
}

