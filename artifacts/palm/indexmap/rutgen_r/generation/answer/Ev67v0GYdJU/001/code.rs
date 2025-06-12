// Answer 0

#[test]
fn test_insert_with_valid_value() {
    struct Entry<V> {
        value: Option<V>,
    }

    impl<V> Entry<V> {
        pub fn get_mut(&mut self) -> &mut Option<V> {
            &mut self.value
        }

        pub fn insert(&mut self, value: V) -> V {
            std::mem::replace(self.get_mut(), Some(value))
                .unwrap_or_else(|| panic!("Expected an old value"))
        }
    }

    let mut entry = Entry { value: None };
    let old_value = entry.insert(42);
    assert!(old_value.is_none());

    let old_value = entry.insert(100);
    assert_eq!(old_value, Some(42));
}


#[test]
#[should_panic(expected = "Expected an old value")]
fn test_insert_panic_on_empty_value() {
    struct Entry<V> {
        value: Option<V>,
    }

    impl<V> Entry<V> {
        pub fn get_mut(&mut self) -> &mut Option<V> {
            &mut self.value
        }

        pub fn insert(&mut self, value: V) -> V {
            std::mem::replace(self.get_mut(), Some(value))
                .unwrap_or_else(|| panic!("Expected an old value"))
        }
    }

    let mut entry = Entry { value: None };
    // Simulate panic by trying to get the previous value of an uninitialized entry
    let _ = entry.insert("test");
}

#[test]
fn test_insert_overwrite_value() {
    struct Entry<V> {
        value: Option<V>,
    }

    impl<V> Entry<V> {
        pub fn get_mut(&mut self) -> &mut Option<V> {
            &mut self.value
        }

        pub fn insert(&mut self, value: V) -> V {
            std::mem::replace(self.get_mut(), Some(value))
                .unwrap_or_else(|| panic!("Expected an old value"))
        }
    }

    let mut entry = Entry { value: Some("initial") };
    let old_value = entry.insert("new_value");
    assert_eq!(old_value, Some("initial"));
    assert_eq!(entry.value, Some("new_value"));
}

