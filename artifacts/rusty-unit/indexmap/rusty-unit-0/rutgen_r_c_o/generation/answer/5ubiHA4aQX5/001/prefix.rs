// Answer 0

#[test]
fn test_new_with_non_empty_iter() {
    let mut buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 1, key: 10, value: 100 },
        Bucket { hash: 2, key: 20, value: 200 },
    ];
    let drain = buckets.drain(..);
    let _drain_instance = Drain::new(drain);
}

#[test]
fn test_new_with_empty_iter() {
    let buckets: Vec<Bucket<i32, i32>> = vec![];
    let drain = buckets.drain(..);
    let _drain_instance = Drain::new(drain);
}

#[test]
fn test_new_with_large_iter() {
    let mut buckets: Vec<Bucket<i32, i32>> = (0..1000).map(|i| Bucket { hash: i as u64, key: i, value: i * 10 }).collect();
    let drain = buckets.drain(..);
    let _drain_instance = Drain::new(drain);
}

#[test]
#[should_panic]
fn test_new_with_drain_on_iterable_exceeding_capacity() {
    let mut buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 1, key: 10, value: 100 },
    ];
    let drain = buckets.drain(..);
    let _drain_instance = Drain::new(drain);
    buckets.push(Bucket { hash: 2, key: 20, value: 200 });
}

