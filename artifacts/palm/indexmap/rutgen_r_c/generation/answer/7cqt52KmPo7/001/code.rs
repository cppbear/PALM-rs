// Answer 0

#[test]
fn test_values_mut_debug_empty() {
    let buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let slice: &[Bucket<i32, i32>] = &buckets;
    let iter_mut = ValuesMut {
        iter: slice.iter_mut(),
    };
    let result = format!("{:?}", iter_mut);
    assert_eq!(result, "[]");
}

#[test]
fn test_values_mut_debug_single() {
    let bucket = Bucket { hash: 0, key: 1, value: 42 };
    let buckets = vec![bucket];
    let slice: &[Bucket<i32, i32>] = &buckets;
    let iter_mut = ValuesMut {
        iter: slice.iter_mut(),
    };
    let result = format!("{:?}", iter_mut);
    assert_eq!(result, "[42]");
}

#[test]
fn test_values_mut_debug_multiple() {
    let buckets = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];
    let slice: &[Bucket<i32, i32>] = &buckets;
    let iter_mut = ValuesMut {
        iter: slice.iter_mut(),
    };
    let result = format!("{:?}", iter_mut);
    assert_eq!(result, "[10, 20, 30]");
}

