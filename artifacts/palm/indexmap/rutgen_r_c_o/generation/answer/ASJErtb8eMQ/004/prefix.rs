// Answer 0

#[test]
fn test_get_disjoint_opt_mut_success_case() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: "value0" },
            Bucket { hash: 1, key: 1, value: "value1" },
            Bucket { hash: 2, key: 2, value: "value2" },
            Bucket { hash: 3, key: 3, value: "value3" },
            Bucket { hash: 4, key: 4, value: "value4" },
            Bucket { hash: 5, key: 5, value: "value5" },
            Bucket { hash: 6, key: 6, value: "value6" },
            Bucket { hash: 7, key: 7, value: "value7" },
            Bucket { hash: 8, key: 8, value: "value8" },
            Bucket { hash: 9, key: 9, value: "value9" },
        ]
    };

    let indices = [Some(0), Some(1), Some(2)];
    let _ = slice.get_disjoint_opt_mut(indices);
}

#[test]
fn test_get_disjoint_opt_mut_no_overlap() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: "value0" },
            Bucket { hash: 1, key: 1, value: "value1" },
            Bucket { hash: 2, key: 2, value: "value2" },
            Bucket { hash: 3, key: 3, value: "value3" },
            Bucket { hash: 4, key: 4, value: "value4" },
            Bucket { hash: 5, key: 5, value: "value5" },
            Bucket { hash: 6, key: 6, value: "value6" },
            Bucket { hash: 7, key: 7, value: "value7" },
            Bucket { hash: 8, key: 8, value: "value8" },
            Bucket { hash: 9, key: 9, value: "value9" },
        ]
    };

    let indices = [Some(2), Some(4), Some(6)];
    let _ = slice.get_disjoint_opt_mut(indices);
}

#[test]
fn test_get_disjoint_opt_mut_empty_indices() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: "value0" },
            Bucket { hash: 1, key: 1, value: "value1" },
        ]
    };

    let indices: [Option<usize>; 0] = [];
    let _ = slice.get_disjoint_opt_mut(indices);
}

#[test]
#[should_panic(expected = "IndexOutOfBounds")]
fn test_get_disjoint_opt_mut_index_out_of_bounds() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: "value0" },
            Bucket { hash: 1, key: 1, value: "value1" },
        ]
    };

    let indices = [Some(0), Some(2)];
    let _ = slice.get_disjoint_opt_mut(indices);
}

#[test]
#[should_panic(expected = "OverlappingIndices")]
fn test_get_disjoint_opt_mut_overlapping_indices() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: 0, value: "value0" },
            Bucket { hash: 1, key: 1, value: "value1" },
        ]
    };

    let indices = [Some(0), Some(0)];
    let _ = slice.get_disjoint_opt_mut(indices);
}

