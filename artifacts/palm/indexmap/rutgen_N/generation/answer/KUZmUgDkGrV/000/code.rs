// Answer 0

#[test]
fn test_from_slice() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        pub(super) const fn from_slice(entries: &[Bucket<K, V>]) -> &Self {
            unsafe { &*(entries as *const [Bucket<K, V>] as *const Self) }
        }
    }

    let entries = [
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];
    
    let map: &TestMap<i32, &str> = TestMap::from_slice(&entries);
    
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].key, 1);
    assert_eq!(map.entries[0].value, "a");
    assert_eq!(map.entries[1].key, 2);
    assert_eq!(map.entries[1].value, "b");
}

