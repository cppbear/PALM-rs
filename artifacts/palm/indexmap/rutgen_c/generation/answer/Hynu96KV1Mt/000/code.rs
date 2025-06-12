// Answer 0

#[test]
fn test_from_slice_empty() {
    let entries: &[Bucket<i32>] = &[];
    let slice = Slice::from_slice(entries);
    assert_eq!(slice.entries.len(), 0);
}

#[test]
fn test_from_slice_single_element() {
    let entries = [Bucket { hash: 0, key: 1, value: "a" }];
    let slice = Slice::from_slice(&entries);
    assert_eq!(slice.entries.len(), 1);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, "a");
}

#[test]
fn test_from_slice_multiple_elements() {
    let entries = [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
        Bucket { hash: 2, key: 3, value: "c" },
    ];
    let slice = Slice::from_slice(&entries);
    assert_eq!(slice.entries.len(), 3);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[1].value, "b");
}

#[should_panic]
fn test_from_slice_invalid_pointer() {
    let invalid_entries: &[_] = core::slice::from_raw_parts(core::ptr::null(), 1);
    Slice::from_slice(invalid_entries);
}

