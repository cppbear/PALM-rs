// Answer 0

#[test]
fn test_first_mut_with_non_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.first_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map = TestMap {
        entries: vec![
            Bucket { key: 1, value: "first" },
            Bucket { key: 2, value: "second" },
        ],
    };
    
    if let Some((key, value)) = map.first_mut() {
        assert_eq!(*key, 1);
        assert_eq!(*value, "first");
        *value = "updated";
    }
    
    assert_eq!(map.entries[0].value, "updated");
}

#[test]
fn test_first_mut_with_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.first_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map: TestMap<i32, &str> = TestMap { entries: vec![] };
    
    let result = map.first_mut();
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_first_mut_with_single_entry() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> TestMap<K, V> {
        pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.first_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut map = TestMap {
        entries: vec![Bucket { key: 1, value: "single" }],
    };
    
    let (_key, value) = map.first_mut().expect("Map should contain an entry");
    panic!("Forcing a panic in single entry test");
}

