// Answer 0

#[test]
fn test_is_empty_when_empty() {
    struct MapSlice {
        entries: Vec<i32>,
    }

    let map_slice = MapSlice { entries: Vec::new() };
    assert!(map_slice.is_empty());
}

#[test]
fn test_is_empty_when_not_empty() {
    struct MapSlice {
        entries: Vec<i32>,
    }

    let map_slice = MapSlice { entries: vec![1, 2, 3] };
    assert!(!map_slice.is_empty());
}

