// Answer 0

#[test]
fn test_is_empty_with_empty_slice() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    let set_slice = SetSlice { entries: Vec::new() };
    assert!(set_slice.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_slice() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    let set_slice = SetSlice { entries: vec![1, 2, 3] };
    assert!(!set_slice.is_empty());
}

#[test]
fn test_is_empty_with_large_slice() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    let set_slice = SetSlice { entries: (0..10000).collect() };
    assert!(!set_slice.is_empty());
}

#[test]
fn test_is_empty_with_single_element() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    let set_slice = SetSlice { entries: vec![10] };
    assert!(!set_slice.is_empty());
}

#[test]
fn test_is_empty_with_zero_capacity() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    let set_slice = SetSlice { entries: Vec::with_capacity(0) };
    assert!(set_slice.is_empty());
}

