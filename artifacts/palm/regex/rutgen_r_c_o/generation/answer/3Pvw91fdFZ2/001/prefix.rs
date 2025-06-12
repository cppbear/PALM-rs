// Answer 0

#[test]
fn test_index_valid_capture() {
    let text = "Hello, World!";
    let locs = Locations(vec![Slot { start: 0, end: 5 }, Slot { start: 7, end: 12 }]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let result_0 = captures.index(0);
    let result_1 = captures.index(1);
}

#[test]
#[should_panic(expected = "no group at index '2'")]
fn test_index_out_of_bounds() {
    let text = "Hello, World!";
    let locs = Locations(vec![Slot { start: 0, end: 5 }, Slot { start: 7, end: 12 }]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let result = captures.index(2);
}

#[test]
fn test_index_empty_captures() {
    let text = "Hello, World!";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures { text, locs, named_groups };

    let result = captures.index(0);
}

