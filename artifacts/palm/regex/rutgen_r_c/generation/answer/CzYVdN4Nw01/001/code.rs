// Answer 0

#[test]
fn test_iter_with_non_empty_captures() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![Slot { /* Initialize with appropriate fields */ }]);
    let text = "test string";
    let named_groups = Arc::new(HashMap::new());

    let caps = TestCaptures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.text, text);
}

#[test]
fn test_iter_with_empty_locations() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![]);
    let text = "empty locations";
    let named_groups = Arc::new(HashMap::new());

    let caps = TestCaptures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.text, text);
    assert_eq!(iter.it.locs.len(), 0);
}

#[test]
fn test_iter_with_multiple_locations() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let locs = Locations(vec![
        Slot { /* Initialize first slot */ },
        Slot { /* Initialize second slot */ },
    ]);
    let text = "test multiple locations";
    let named_groups = Arc::new(HashMap::new());

    let caps = TestCaptures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.text, text);
    assert_eq!(iter.it.locs.len(), 2);
}

