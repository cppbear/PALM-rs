// Answer 0

#[test]
fn test_captures_len_non_empty() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct TestSlot;
    struct TestLocations(Vec<TestSlot>);

    impl Locations {
        pub fn new() -> Self {
            Locations(vec![TestSlot, TestSlot])
        }
    }

    let named_groups = Arc::new(HashMap::new());
    let locs = TestLocations::new();
    let captures = Captures {
        text: b"abc",
        locs,
        named_groups,
    };

    assert_eq!(captures.len(), 1);
}

#[test]
fn test_captures_len_zero() {
    struct TestSlot;
    struct TestLocations(Vec<TestSlot>);

    impl Locations {
        pub fn new() -> Self {
            Locations(vec![])
        }
    }

    let named_groups = Arc::new(HashMap::new());
    let locs = TestLocations::new();
    let captures = Captures {
        text: b"abc",
        locs,
        named_groups,
    };

    assert_eq!(captures.len(), 0);
}

