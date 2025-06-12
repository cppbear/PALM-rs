// Answer 0

#[test]
fn test_new_with_non_empty_iter() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct NewStruct<'a, K, V> {
        iter: std::vec::Drain<'a, Bucket<K, V>>,
    }

    fn new<'a, K, V>(iter: std::vec::Drain<'a, Bucket<K, V>>) -> NewStruct<'a, K, V> {
        NewStruct { iter }
    }

    let mut vec_buckets = vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];
    let iter = vec_buckets.drain(..);
    
    let result = new(iter);
    
    assert!(result.iter.len() == 2);
}

#[test]
fn test_new_with_empty_iter() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct NewStruct<'a, K, V> {
        iter: std::vec::Drain<'a, Bucket<K, V>>,
    }

    fn new<'a, K, V>(iter: std::vec::Drain<'a, Bucket<K, V>>) -> NewStruct<'a, K, V> {
        NewStruct { iter }
    }

    let mut vec_buckets: Vec<Bucket<i32, &str>> = Vec::new();
    let iter = vec_buckets.drain(..);
    
    let result = new(iter);
    
    assert!(result.iter.len() == 0);
}

#[should_panic]
#[test]
fn test_new_with_iter_after_vec_dropped() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct NewStruct<'a, K, V> {
        iter: std::vec::Drain<'a, Bucket<K, V>>,
    }

    fn new<'a, K, V>(iter: std::vec::Drain<'a, Bucket<K, V>>) -> NewStruct<'a, K, V> {
        NewStruct { iter }
    }

    let iter;
    {
        let mut vec_buckets = vec![
            Bucket { key: 1, value: "x" },
        ];
        iter = vec_buckets.drain(..);
    } // vec_buckets is dropped here
    
    let _result = new(iter); // This should trigger a panic due to use after drop
}

