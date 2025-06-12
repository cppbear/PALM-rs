// Answer 0

#[test]
fn test_insert_with_existing_value() {
    struct Entry {
        value: Option<i32>,
    }

    impl Entry {
        fn new(value: Option<i32>) -> Self {
            Entry { value }
        }

        fn get_mut(&mut self) -> &mut Option<i32> {
            &mut self.value
        }

        fn insert(&mut self, value: i32) -> Option<i32> {
            std::mem::replace(self.get_mut(), Some(value))
        }
    }

    let mut entry = Entry::new(Some(42));
    let old_value = entry.insert(100);
    assert_eq!(old_value, Some(42));
    assert_eq!(entry.value, Some(100));
}

#[test]
fn test_insert_with_none_value() {
    struct Entry {
        value: Option<i32>,
    }

    impl Entry {
        fn new(value: Option<i32>) -> Self {
            Entry { value }
        }

        fn get_mut(&mut self) -> &mut Option<i32> {
            &mut self.value
        }

        fn insert(&mut self, value: i32) -> Option<i32> {
            std::mem::replace(self.get_mut(), Some(value))
        }
    }

    let mut entry = Entry::new(None);
    let old_value = entry.insert(50);
    assert_eq!(old_value, None);
    assert_eq!(entry.value, Some(50));
}

#[test]
#[should_panic]
fn test_insert_panic_on_uninitialised_value() {
    struct Entry {
        value: Option<i32>,
    }

    impl Entry {
        fn new() -> Self {
            Entry { value: None }
        }

        fn get_mut(&mut self) -> &mut Option<i32> {
            &mut self.value
        }

        fn insert(&mut self, value: i32) -> Option<i32> {
            std::mem::replace(self.get_mut(), Some(value))
        }
    }

    let mut entry = Entry::new();
    let _ = entry.insert(10);
    let _ = entry.insert(20); // This one will panic on trying to access current value.
}

