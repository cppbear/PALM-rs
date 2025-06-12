// Answer 0

#[test]
fn test_get_range_mut_empty() {
    let mut slice = Slice::new_mut();
    let range = 0..0;
    slice.get_range_mut(range);
}

#[test]
fn test_get_range_mut_single_element() {
    let mut slice = Slice::new_mut();
    slice.entries = [Bucket { hash: 0, key: "a", value: 1 }];
    let range = 0..1;
    slice.get_range_mut(range);
}

#[test]
fn test_get_range_mut_valid_range() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
    ];
    let range = 0..2;
    slice.get_range_mut(range);
}

#[test]
fn test_get_range_mut_partial_range() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
    ];
    let range = 0..1;
    slice.get_range_mut(range);
}

#[test]
fn test_get_range_mut_exclusive_start() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
    ];
    let range = 1..2;
    slice.get_range_mut(range);
}

#[test]
fn test_get_range_mut_invalid_range() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
    ];
    let range = 2..3;
    let result = slice.get_range_mut(range);
    assert!(result.is_none());
}

#[test]
fn test_get_range_mut_full_inclusive() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
        Bucket { hash: 2, key: "c", value: 3 },
    ];
    let range = 0..=2;
    slice.get_range_mut(range);
}

#[test]
fn test_get_range_mut_full_exclusive() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
        Bucket { hash: 2, key: "c", value: 3 },
    ];
    let range = 0..3;
    slice.get_range_mut(range);
}

#[test]
#[should_panic]
fn test_get_range_mut_out_of_bounds() {
    let mut slice = Slice::new_mut();
    slice.entries = [
        Bucket { hash: 0, key: "a", value: 1 },
    ];
    let range = 1..2;
    slice.get_range_mut(range);
}

