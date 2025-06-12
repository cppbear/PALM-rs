// Answer 0

#[test]
fn test_iter_valid_captures() {
    struct TestCaptures<'a> {
        text: &'a str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = TestCaptures {
        text: "test input",
        locs,
        named_groups,
    };

    let mut it = captures.iter();
    assert_eq!(it.caps.text, captures.text);
}

#[test]
fn test_iter_empty_locations() {
    struct TestCaptures<'a> {
        text: &'a str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = TestCaptures {
        text: "empty input",
        locs,
        named_groups,
    };

    let it = captures.iter();
    assert_eq!(it.caps.text, captures.text);
}

