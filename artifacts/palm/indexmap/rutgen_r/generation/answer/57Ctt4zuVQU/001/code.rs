// Answer 0

#[test]
fn test_and_modify_entry_not_occupied() {
    struct TestEntry<V> {
        is_occupied: bool,
        value: Option<V>,
    }

    impl<V> TestEntry<V> {
        fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut V),
        {
            if self.is_occupied {
                // Not necessary to trigger panic here as per constraints
            }
            self
        }
    }

    let entry = TestEntry {
        is_occupied: false,
        value: None,
    };

    let modified_entry = entry.and_modify(|_v| {
        panic!("This code should not run as the entry is not occupied");
    });

    assert_eq!(modified_entry.is_occupied, false);
}

#[test]
fn test_and_modify_entry_occupied_return_self() {
    struct TestEntry<V> {
        is_occupied: bool,
        value: Option<V>,
    }

    impl<V> TestEntry<V> {
        fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut V),
        {
            if self.is_occupied {
                if let Some(ref mut v) = self.value {
                    f(v);
                }
            }
            self
        }
    }

    let mut entry = TestEntry {
        is_occupied: true,
        value: Some(10),
    };

    let modified_entry = entry.and_modify(|v| {
        *v += 5; // Modify the value
    });

    assert_eq!(modified_entry.value, Some(15));
}

