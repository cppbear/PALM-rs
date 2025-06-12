// Answer 0

#[test]
fn test_get_valid_index() {
    struct MockEntry {
        value: i32,
    }
    
    struct MockMap {
        entries: Vec<MockEntry>,
    }

    struct Entry<'a> {
        map: &'a MockMap,
        index: usize,
    }

    let map = MockMap {
        entries: vec![MockEntry { value: 1 }, MockEntry { value: 2 }],
    };
    
    let entry = Entry { map: &map, index: 0 };
    assert_eq!(entry.get(), &1);
}

#[test]
#[should_panic]
fn test_get_out_of_bounds_index() {
    struct MockEntry {
        value: i32,
    }
    
    struct MockMap {
        entries: Vec<MockEntry>,
    }

    struct Entry<'a> {
        map: &'a MockMap,
        index: usize,
    }

    let map = MockMap {
        entries: vec![MockEntry { value: 1 }, MockEntry { value: 2 }],
    };

    // Attempt to access an index that is out of bounds
    let entry = Entry { map: &map, index: 2 };
    let _ = entry.get(); // This should panic
}

#[test]
fn test_get_empty_map() {
    struct MockEntry {
        value: i32,
    }
    
    struct MockMap {
        entries: Vec<MockEntry>,
    }

    struct Entry<'a> {
        map: &'a MockMap,
        index: usize,
    }

    let map = MockMap { entries: vec![] };

    // Attempt to access any index in an empty map, which should panic
    let entry = Entry { map: &map, index: 0 };
    let _ = entry.get(); // This should panic as well
}

