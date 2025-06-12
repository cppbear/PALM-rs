// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let mut buckets: Vec<Bucket<i32, i32>> = Vec::with_capacity(10);
    for i in 0..10 {
        buckets.push(Bucket { hash: HashValue::default(), key: i, value: i * 2 });
    }
    let drain = Drain::new(buckets.drain(..));
    let slice = drain.as_slice();
}

#[test]
fn test_as_slice_empty() {
    let buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let drain = Drain::new(buckets.drain(..));
    let slice = drain.as_slice();
}

#[test]
fn test_as_slice_max_capacity() {
    let mut buckets: Vec<Bucket<i32, i32>> = Vec::with_capacity(1000);
    for i in 0..1000 {
        buckets.push(Bucket { hash: HashValue::default(), key: i, value: i * 3 });
    }
    let drain = Drain::new(buckets.drain(..));
    let slice = drain.as_slice();
}

#[test]
fn test_as_slice_edge_case() {
    let mut buckets: Vec<Bucket<i32, i32>> = Vec::with_capacity(1);
    buckets.push(Bucket { hash: HashValue::default(), key: 0, value: 0 });
    let drain = Drain::new(buckets.drain(..));
    let slice = drain.as_slice();
}

