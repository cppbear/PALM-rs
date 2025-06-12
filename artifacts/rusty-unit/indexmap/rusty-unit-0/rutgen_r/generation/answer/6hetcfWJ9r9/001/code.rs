// Answer 0

#[test]
fn test_insert_key_updates_key() {
    struct TestEntry {
        key: Option<i32>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry { key: None }
        }

        fn key_mut(&mut self) -> &mut Option<i32> {
            &mut self.key
        }

        fn insert_key(&mut self, key: i32) -> Option<i32> {
            std::mem::replace(self.key_mut(), Some(key))
        }
    }

    let mut entry = TestEntry::new();
    let old_key = entry.insert_key(42);
    assert_eq!(old_key, None);
    assert_eq!(entry.key, Some(42));
    
    let old_key = entry.insert_key(100);
    assert_eq!(old_key, Some(42));
    assert_eq!(entry.key, Some(100));
}

#[test]
fn test_insert_key_with_none() {
    struct TestEntry {
        key: Option<i32>,
    }

    impl TestEntry {
        fn new() -> Self {
            TestEntry { key: None }
        }

        fn key_mut(&mut self) -> &mut Option<i32> {
            &mut self.key
        }

        fn insert_key(&mut self, key: i32) -> Option<i32> {
            std::mem::replace(self.key_mut(), Some(key))
        }
    }

    let mut entry = TestEntry::new();
    let old_key = entry.insert_key(0);
    assert_eq!(old_key, None);
    assert_eq!(entry.key, Some(0));
    
    let old_key = entry.insert_key(-1);
    assert_eq!(old_key, Some(0));
    assert_eq!(entry.key, Some(-1));
}

