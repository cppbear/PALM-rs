// Answer 0

#[test]
fn test_index_valid() {
    let text = "test text";
    let locs = Locations(vec![Slot(0, 4), Slot(5, 9)]); // assuming Slot is defined such that it takes start and end positions
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    
    let result = captures.index(0);
    assert_eq!(result, "test"); // assuming Slot(0, 4) corresponds to "test"
}

#[test]
#[should_panic(expected = "no group at index '2'")]
fn test_index_out_of_bounds() {
    let text = "test text";
    let locs = Locations(vec![Slot(0, 4), Slot(5, 9)]);
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };
    
    let _result = captures.index(2); // this should panic
}

