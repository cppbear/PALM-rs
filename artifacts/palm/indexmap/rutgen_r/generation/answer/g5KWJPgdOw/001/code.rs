// Answer 0

#[test]
fn test_is_empty_on_empty_slice() {
    struct MapSlice {
        entries: Vec<i32>,
    }
    
    let slice = MapSlice { entries: Vec::new() };
    assert!(slice.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_slice() {
    struct MapSlice {
        entries: Vec<i32>,
    }
    
    let slice = MapSlice { entries: vec![1, 2, 3] };
    assert!(!slice.is_empty());
}

#[test]
fn test_is_empty_on_large_slice() {
    struct MapSlice {
        entries: Vec<i32>,
    }
    
    let slice = MapSlice { entries: (0..1000).collect() };
    assert!(!slice.is_empty());
}

#[test]
fn test_is_empty_on_single_element_slice() {
    struct MapSlice {
        entries: Vec<i32>,
    }
    
    let slice = MapSlice { entries: vec![42] };
    assert!(!slice.is_empty());
}

