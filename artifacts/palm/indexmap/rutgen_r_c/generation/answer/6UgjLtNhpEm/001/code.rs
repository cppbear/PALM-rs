// Answer 0

#[test]
fn test_first_non_empty() {
    struct MockIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl MockIndexMap {
        fn as_entries(&self) -> &Vec<Bucket<i32, String>> {
            &self.entries
        }

        fn first(&self) -> Option<(&i32, &String)> {
            self.as_entries().first().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = MockIndexMap {
        entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: "Value1".to_string() },
            Bucket { hash: HashValue(2), key: 2, value: "Value2".to_string() },
        ],
    };

    let result = map.first();
    assert_eq!(result, Some((&1, &"Value1".to_string())));
}

#[test]
fn test_first_empty() {
    struct MockIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl MockIndexMap {
        fn as_entries(&self) -> &Vec<Bucket<i32, String>> {
            &self.entries
        }

        fn first(&self) -> Option<(&i32, &String)> {
            self.as_entries().first().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let map = MockIndexMap {
        entries: Vec::new(),
    };

    let result = map.first();
    assert_eq!(result, None);
}

