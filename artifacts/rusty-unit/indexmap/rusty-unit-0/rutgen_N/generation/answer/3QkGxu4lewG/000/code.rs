// Answer 0

#[test]
fn test_from_mut_slice() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }
    
    struct IndexMap<K, V> {
        buckets: Vec<Bucket<K, V>>,
    }

    let mut entries: [Bucket<i32, &str>; 3] = [
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
        Bucket { key: 3, value: "three" },
    ];

    let index_map: &mut IndexMap<i32, &str> = from_mut_slice(&mut entries);
    
    assert_eq!(index_map.buckets.len(), 3);
    assert_eq!(index_map.buckets[0].key, 1);
    assert_eq!(index_map.buckets[0].value, "one");
}

unsafe fn from_mut_slice<K, V>(entries: &mut [Bucket<K, V>]) -> &mut IndexMap<K, V> {
    &mut *(entries as *mut [Bucket<K, V>] as *mut IndexMap<K, V>)
}

