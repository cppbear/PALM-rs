// Answer 0

#[test]
fn test_drain_new() {
    let buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 1, key: 1, value: 10 },
        Bucket { hash: 2, key: 2, value: 20 },
        Bucket { hash: 3, key: 3, value: 30 },
    ];
    let mut drain_iter = buckets.clone().into_iter().collect::<Vec<_>>().drain(..);
    let drain = Drain::new(drain_iter);

    assert!(drain.iter.len() == buckets.len());
}

#[test]
#[should_panic]
fn test_drain_new_empty() {
    let empty_buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let mut empty_drain_iter = empty_buckets.clone().into_iter().collect::<Vec<_>>().drain(..);
    let drain = Drain::new(empty_drain_iter);

    assert!(drain.iter.len() == empty_buckets.len());
}

