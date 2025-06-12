// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

#[test]
fn test_into_boxed() {
    let bucket1 = Bucket { key: 1, value: "a" };
    let bucket2 = Bucket { key: 2, value: "b" };
    let buckets: Vec<Bucket<_, _>> = vec![bucket1, bucket2];
    let boxed_buckets: Box<[Bucket<_, _>]> = Box::new(buckets);
    
    let result = boxed_buckets.into_boxed();
    
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].key, 1);
    assert_eq!(result[0].value, "a");
    assert_eq!(result[1].key, 2);
    assert_eq!(result[1].value, "b");
}

#[test]
#[should_panic]
fn test_into_boxed_empty() {
    let buckets: Vec<Bucket<_, _>> = vec![];
    let boxed_buckets: Box<[Bucket<_, _>]> = Box::new(buckets);
    
    let result = boxed_buckets.into_boxed();
    
    assert_eq!(result.len(), 0);
}

