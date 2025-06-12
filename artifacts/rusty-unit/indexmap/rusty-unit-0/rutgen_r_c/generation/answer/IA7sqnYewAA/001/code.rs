// Answer 0

#[test]
fn test_partition_point_with_true_predicate() {
    let entries = [
        Bucket { hash: 1, key: 1, value: "a" },
        Bucket { hash: 2, key: 2, value: "b" },
        Bucket { hash: 3, key: 3, value: "c" },
    ];
    let slice = Slice { entries };

    let result = slice.partition_point(|_, _| true);
    assert_eq!(result, 3);
}

#[test]
fn test_partition_point_with_false_predicate() {
    let entries = [
        Bucket { hash: 1, key: 1, value: "a" },
        Bucket { hash: 2, key: 2, value: "b" },
        Bucket { hash: 3, key: 3, value: "c" },
    ];
    let slice = Slice { entries };

    let result = slice.partition_point(|_, _| false);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_with_some_false_and_true() {
    let entries = [
        Bucket { hash: 1, key: 1, value: "a" },
        Bucket { hash: 2, key: 2, value: "b" },
        Bucket { hash: 3, key: 3, value: "c" },
        Bucket { hash: 4, key: 4, value: "d" },
    ];
    let slice = Slice { entries };

    let result = slice.partition_point(|key, _| *key < 3);
    assert_eq!(result, 2);
}

#[test]
fn test_partition_point_with_empty_slice() {
    let entries: [Bucket<u32, &str>; 0] = [];
    let slice = Slice { entries };

    let result = slice.partition_point(|_, _| true);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_with_boundary_value() {
    let entries = [
        Bucket { hash: 1, key: 1, value: "a" },
        Bucket { hash: 2, key: 2, value: "b" },
        Bucket { hash: 3, key: 3, value: "c" },
    ];
    let slice = Slice { entries };

    let result = slice.partition_point(|key, _| *key <= 2);
    assert_eq!(result, 2);
}

