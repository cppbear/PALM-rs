// Answer 0

#[test]
fn test_drain_with_valid_input() {
    let mut buckets = Vec::new();
    for i in 1..=1000 {
        buckets.push(Bucket { hash: 0, key: i, value: i * 2 });
    }
    let drain = buckets.drain(..);
    let _result = Drain::new(drain);
}

#[test]
fn test_drain_with_minimum_capacity() {
    let mut buckets = Vec::new();
    buckets.push(Bucket { hash: 0, key: 1, value: 1 });
    let drain = buckets.drain(..);
    let _result = Drain::new(drain);
}

#[test]
fn test_drain_with_edge_capacity() {
    let mut buckets = Vec::new();
    for i in 1..=999 {
        buckets.push(Bucket { hash: 0, key: i, value: 2000 - i });
    }
    let drain = buckets.drain(..);
    let _result = Drain::new(drain);
}

#[test]
fn test_drain_with_reaching_limit() {
    let mut buckets = Vec::new();
    for i in 1..=1000 {
        buckets.push(Bucket { hash: 0, key: i, value: 1000 - i });
    }
    let drain = buckets.drain(..);
    let _result = Drain::new(drain);
}

#[test]
fn test_drain_with_non_empty_after_creation() {
    let mut buckets = Vec::new();
    for i in 1..=500 {
        buckets.push(Bucket { hash: 0, key: i, value: 1000 - i });
    }
    let drain = buckets.drain(..);
    let _result = Drain::new(drain);
    assert!(!buckets.is_empty());
}

