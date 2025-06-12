// Answer 0

#[test]
fn test_slice_index_valid() {
    let entries = [
        Bucket { hash: 1, key: "a", value: 10 },
        Bucket { hash: 2, key: "b", value: 20 },
        Bucket { hash: 3, key: "c", value: 30 },
    ];
    let slice = Slice { entries };

    assert_eq!(slice[0], "a");
    assert_eq!(slice[1], "b");
    assert_eq!(slice[2], "c");
}

#[should_panic]
fn test_slice_index_out_of_bounds_lower() {
    let entries = [
        Bucket { hash: 1, key: "a", value: 10 },
        Bucket { hash: 2, key: "b", value: 20 },
        Bucket { hash: 3, key: "c", value: 30 },
    ];
    let slice = Slice { entries };

    // Accessing index -1 should panic
    let _ = &slice[-1];
}

#[should_panic]
fn test_slice_index_out_of_bounds_upper() {
    let entries = [
        Bucket { hash: 1, key: "a", value: 10 },
        Bucket { hash: 2, key: "b", value: 20 },
        Bucket { hash: 3, key: "c", value: 30 },
    ];
    let slice = Slice { entries };

    // Accessing index 3 should panic (since valid indices are 0, 1, 2)
    let _ = &slice[3];
}

