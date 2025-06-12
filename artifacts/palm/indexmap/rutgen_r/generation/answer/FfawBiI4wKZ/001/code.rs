// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    struct Entry<V> {
        value: Option<V>,
    }

    impl<V> Entry<V> {
        fn vacant() -> Self {
            Entry { value: None }
        }

        fn occupied(value: V) -> Self {
            Entry { value: Some(value) }
        }

        fn or_insert(&mut self, default: V) -> &mut V {
            match self.value {
                Some(ref mut v) => v,
                None => {
                    self.value = Some(default);
                    self.value.as_mut().unwrap() // unwrap is safe here since we just inserted
                }
            }
        }
    }

    let mut entry = Entry::vacant();
    let default_value = 42; // Example default value, could be anything
    let value_ref = entry.or_insert(default_value);
    
    assert_eq!(*value_ref, 42);
    assert_eq!(entry.value, Some(42));
}

#[test]
fn test_or_insert_occupied_entry() {
    struct Entry<V> {
        value: Option<V>,
    }

    impl<V> Entry<V> {
        fn occupied(value: V) -> Self {
            Entry { value: Some(value) }
        }

        fn or_insert(&mut self, default: V) -> &mut V {
            match self.value {
                Some(ref mut v) => v,
                None => {
                    self.value = Some(default);
                    self.value.as_mut().unwrap() // unwrap is safe here since we just inserted
                }
            }
        }
    }

    let mut entry = Entry::occupied(30);
    let default_value = 42; 
    let value_ref = entry.or_insert(default_value);
    
    assert_eq!(*value_ref, 30); // Should return the already existing value, not the default
    assert_eq!(entry.value, Some(30));
}

