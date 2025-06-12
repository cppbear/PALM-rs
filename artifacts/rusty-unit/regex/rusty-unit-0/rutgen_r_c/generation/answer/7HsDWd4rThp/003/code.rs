// Answer 0

#[test]
fn test_fmt_with_empty_captures() {
    use std::collections::HashMap;
    use std::fmt::Formatter;

    struct EmptyLocations;

    impl Locations {
        fn new() -> Self {
            Locations(Vec::new())
        }
    }

    let named_groups = Arc::new(HashMap::new());
    let text = b"";
    let locs = EmptyLocations.new();
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: named_groups,
    };

    let mut formatter = Formatter::new();
    let result = fmt(&captures, &mut formatter);

    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_single_named_capture() {
    use std::collections::HashMap;
    use std::fmt::Formatter;

    struct SingleSlotLocations;

    impl Locations {
        fn new() -> Self {
            Locations(vec![Some((0, 3))]) // Simulating one capture matching "foo"
        }
    }

    let named_groups = Arc::new(HashMap::from([("name1".to_string(), 0)]));
    let text = b"foo";
    let locs = SingleSlotLocations.new();
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: named_groups,
    };

    let mut formatter = Formatter::new();
    let result = fmt(&captures, &mut formatter);

    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_multiple_captures() {
    use std::collections::HashMap;
    use std::fmt::Formatter;

    struct MultipleSlotsLocations;

    impl Locations {
        fn new() -> Self {
            Locations(vec![
                Some((0, 3)), // "foo"
                Some((4, 7)), // "bar"
                None, // No match for slot 2
            ])
        }
    }

    let named_groups = Arc::new(HashMap::from([
        ("first".to_string(), 0),
        ("second".to_string(), 1),
    ]));
    let text = b"foo bar";
    let locs = MultipleSlotsLocations.new();
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: named_groups,
    };

    let mut formatter = Formatter::new();
    let result = fmt(&captures, &mut formatter);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_slot_index() {
    use std::collections::HashMap;
    use std::fmt::Formatter;

    struct InvalidSlotLocations;

    impl Locations {
        fn new() -> Self {
            Locations(vec![None]); // No matches at all, should panic
        }
    }

    let named_groups = Arc::new(HashMap::new());
    let text = b"";
    let locs = InvalidSlotLocations.new();
    let captures = Captures {
        text: text,
        locs: locs,
        named_groups: named_groups,
    };

    let mut formatter = Formatter::new();
    fmt(&captures, &mut formatter);
}

