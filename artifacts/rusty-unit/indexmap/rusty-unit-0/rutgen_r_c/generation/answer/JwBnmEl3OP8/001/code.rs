// Answer 0

#[test]
fn test_iter_mut_new_with_non_empty_array() {
    let mut buckets: [Bucket<i32, &str>; 3] = [
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    
    let iter_mut = IterMut::new(&mut buckets);
    
    assert_eq!(iter_mut.iter.len(), buckets.len());
}

#[test]
fn test_iter_mut_new_with_empty_array() {
    let mut buckets: [Bucket<i32, &str>; 0] = [];
    
    let iter_mut = IterMut::new(&mut buckets);
    
    assert_eq!(iter_mut.iter.len(), buckets.len());
}

#[should_panic]
fn test_iter_mut_new_with_null_reference() {
    let mut buckets: Option<&mut [Bucket<i32, &str>]> = None;
    
    if let Some(entries) = buckets {
        IterMut::new(entries);
    } else {
        panic!("Expected a mutable reference, but got None");
    }
}

