// Answer 0

#[test]
fn test_partition_point_even_split() {
    struct TestStruct {
        key: i32,
    }

    let entries = vec![
        TestStruct { key: 1 },
        TestStruct { key: 3 },
        TestStruct { key: 5 },
        TestStruct { key: 7 },
        TestStruct { key: 9 },
    ];
    
    let result = entries.partition_point(|x| x.key < 5);
    assert_eq!(result, 2);
}

#[test]
fn test_partition_point_all_less() {
    struct TestStruct {
        key: i32,
    }

    let entries = vec![
        TestStruct { key: 1 },
        TestStruct { key: 2 },
        TestStruct { key: 3 },
    ];

    let result = entries.partition_point(|x| x.key < 4);
    assert_eq!(result, 3);
}

#[test]
fn test_partition_point_none_less() {
    struct TestStruct {
        key: i32,
    }

    let entries = vec![
        TestStruct { key: 5 },
        TestStruct { key: 6 },
        TestStruct { key: 7 },
    ];

    let result = entries.partition_point(|x| x.key < 5);
    assert_eq!(result, 0);
}

#[test]
fn test_partition_point_all_greater() {
    struct TestStruct {
        key: i32,
    }

    let entries = vec![
        TestStruct { key: 5 },
        TestStruct { key: 6 },
        TestStruct { key: 7 },
    ];

    let result = entries.partition_point(|x| x.key > 7);
    assert_eq!(result, 3);
}

#[test]
fn test_partition_point_boundary_case() {
    struct TestStruct {
        key: i32,
    }

    let entries = vec![
        TestStruct { key: 1 },
        TestStruct { key: 2 },
        TestStruct { key: 3 },
        TestStruct { key: 4 },
        TestStruct { key: 5 },
    ];

    let result = entries.partition_point(|x| x.key <= 3);
    assert_eq!(result, 3);
}

