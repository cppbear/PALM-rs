// Answer 0

fn test_fmt_with_named_groups() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn new(text: &'t str, locs: Locations, named_groups: Arc<HashMap<String, usize>>) -> Self {
            Self { text, locs, named_groups }
        }
    }

    let text = "test string";
    let locs = Locations(vec![(0, 4), (5, 11)]);
    let named_groups = Arc::new(HashMap::from([("group1".to_string(), 0), ("group2".to_string(), 1)]));

    let captures = TestCaptures::new(text, locs, named_groups);
    let captures_debug = CapturesDebug(&captures);

    let mut output = String::new();
    let fmt_result = write!(&mut output, "{:?}", captures_debug);
    
    assert!(fmt_result.is_ok());
    assert!(output.contains("group1"));
    assert!(output.contains("group2"));
}

fn test_fmt_with_no_named_groups() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn new(text: &'t str, locs: Locations, named_groups: Arc<HashMap<String, usize>>) -> Self {
            Self { text, locs, named_groups }
        }
    }

    let text = "another test";
    let locs = Locations(vec![(0, 7), (8, 12)]);
    let named_groups = Arc::new(HashMap::new());

    let captures = TestCaptures::new(text, locs, named_groups);
    let captures_debug = CapturesDebug(&captures);

    let mut output = String::new();
    let fmt_result = write!(&mut output, "{:?}", captures_debug);
    
    assert!(fmt_result.is_ok());
    assert!(output.contains("0"));
    assert!(output.contains("1"));
}

fn test_fmt_with_empty_locations() {
    struct TestCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    impl<'t> Captures<'t> {
        fn new(text: &'t str, locs: Locations, named_groups: Arc<HashMap<String, usize>>) -> Self {
            Self { text, locs, named_groups }
        }
    }

    let text = "no matches";
    let locs = Locations(Vec::new());
    let named_groups = Arc::new(HashMap::from([("group1".to_string(), 0)]));

    let captures = TestCaptures::new(text, locs, named_groups);
    let captures_debug = CapturesDebug(&captures);

    let mut output = String::new();
    let fmt_result = write!(&mut output, "{:?}", captures_debug);
    
    assert!(fmt_result.is_ok());
    assert!(output.is_empty());
}

