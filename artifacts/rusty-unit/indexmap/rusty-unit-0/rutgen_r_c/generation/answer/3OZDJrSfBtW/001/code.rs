// Answer 0

#[test]
fn test_values_mut_new_non_empty() {
    let mut buckets: [Bucket<i32, i32>; 3] = [
        Bucket { hash: 1.into(), key: 1, value: 10 },
        Bucket { hash: 2.into(), key: 2, value: 20 },
        Bucket { hash: 3.into(), key: 3, value: 30 },
    ];
    let values_mut = ValuesMut::new(&mut buckets);
    assert_eq!(values_mut.iter.len(), 3);
}

#[test]
fn test_values_mut_new_empty() {
    let mut buckets: [Bucket<i32, i32>; 0] = [];
    let values_mut = ValuesMut::new(&mut buckets);
    assert_eq!(values_mut.iter.len(), 0);
}

#[should_panic]
fn test_values_mut_new_invalid_reference() {
    let mut buckets: [Bucket<i32, i32>; 3] = [
        Bucket { hash: 1.into(), key: 1, value: 10 },
        Bucket { hash: 2.into(), key: 2, value: 20 },
        Bucket { hash: 3.into(), key: 3, value: 30 },
    ];
    let sliced_buckets: &mut [Bucket<i32, i32>] = &mut buckets[0..3];
    drop(sliced_buckets);
    let _ = ValuesMut::new(sliced_buckets);
}

