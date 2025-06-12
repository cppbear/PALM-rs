// Answer 0

#[test]
fn test_equivalent() {
    struct KeyWrapper<'a>(&'a str);
    
    trait Equivalent<K> {
        fn equivalent(lhs: &Self, rhs: &K) -> bool;
    }

    impl Equivalent<String> for KeyWrapper<'_> {
        fn equivalent(lhs: &Self, rhs: &String) -> bool {
            lhs.0 == rhs.as_str()
        }
    }

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    let entries = vec![
        Bucket { key: String::from("key1"), value: 1 },
        Bucket { key: String::from("key2"), value: 2 },
        Bucket { key: String::from("key3"), value: 3 },
    ];

    let key = KeyWrapper("key2");
    let equivalent_fn = equivalent(&key, &entries);

    assert!(equivalent_fn(&1)); // should compare "key2" with entry 1 ("key2")
    assert!(!equivalent_fn(&0)); // should compare "key2" with entry 0 ("key1")
    assert!(!equivalent_fn(&2)); // should compare "key2" with entry 2 ("key3")
}

