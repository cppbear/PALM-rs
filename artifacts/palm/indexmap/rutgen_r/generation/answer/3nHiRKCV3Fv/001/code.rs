// Answer 0

fn last_mut_test() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Collection<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> Collection<K, V> {
        pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
            self.entries.last_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    #[test]
    fn test_last_mut_with_non_empty_collection() {
        let mut collection = Collection {
            entries: vec![
                Bucket { key: 1, value: "a" },
                Bucket { key: 2, value: "b" },
            ],
        };

        if let Some((key, value)) = collection.last_mut() {
            assert_eq!(*key, 2);
            *value = "c";
        } else {
            panic!("Expected a non-empty collection to return a value.");
        }
        assert_eq!(collection.entries.last().unwrap().value, "c");
    }

    #[test]
    fn test_last_mut_with_empty_collection() {
        let mut collection: Collection<i32, &str> = Collection {
            entries: Vec::new(),
        };

        assert!(collection.last_mut().is_none(), "Expected None for empty collection.");
    }
    
    #[test]
    fn test_last_mut_after_mutate() {
        let mut collection = Collection {
            entries: vec![
                Bucket { key: "key1", value: 10 },
                Bucket { key: "key2", value: 20 },
            ],
        };

        if let Some((_, value)) = collection.last_mut() {
            *value += 5;
        }
        
        assert_eq!(collection.entries.last().unwrap().value, 25);
    }
}

