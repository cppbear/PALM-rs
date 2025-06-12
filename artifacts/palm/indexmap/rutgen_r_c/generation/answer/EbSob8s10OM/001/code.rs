// Answer 0

#[test]
fn test_first_non_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl TestMap {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }
        
        fn first(&self) -> Option<&i32> {
            self.as_entries().first().map(|bucket| &bucket.key)
        }
    }

    let test_map = TestMap {
        entries: vec![
            Bucket { hash: 0, key: 1, value: () },
            Bucket { hash: 0, key: 2, value: () },
        ],
    };

    assert_eq!(test_map.first(), Some(&1));
}

#[test]
fn test_first_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl TestMap {
        fn as_entries(&self) -> &[Bucket<i32, ()>] {
            &self.entries
        }
        
        fn first(&self) -> Option<&i32> {
            self.as_entries().first().map(|bucket| &bucket.key)
        }
    }

    let test_map = TestMap {
        entries: vec![],
    };

    assert_eq!(test_map.first(), None);
}

