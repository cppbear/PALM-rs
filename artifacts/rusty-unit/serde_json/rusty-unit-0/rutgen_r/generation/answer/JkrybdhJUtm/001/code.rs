// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use serde_json::{Map, Value};

    struct MockOccupiedEntry<'a> {
        value: &'a mut Value,
    }

    impl<'a> MockOccupiedEntry<'a> {
        fn into_mut(self) -> &'a mut Value {
            self.value
        }
    }

    struct MockEntry<'a> {
        is_occupied: bool,
        occupied_entry: Option<MockOccupiedEntry<'a>>,
    }

    impl<'a> MockEntry<'a> {
        fn new() -> Self {
            Self {
                is_occupied: true,
                occupied_entry: None,
            }
        }

        fn occupy(mut self, value: &'a mut Value) -> Self {
            self.occupied_entry = Some(MockOccupiedEntry { value });
            self
        }

        fn or_insert_with<F>(self, default: F) -> &'a mut Value
        where
            F: FnOnce() -> Value,
        {
            if self.is_occupied {
                self.occupied_entry.unwrap().into_mut()
            } else {
                panic!("Cannot insert into a vacant entry");
            }
        }
    }

    let mut map = Map::new();
    let key = "serde";
    let mut value = Value::String("old_value".to_string());
    let mock_entry = MockEntry::new().occupy(&mut value);

    let result = mock_entry.or_insert_with(|| Value::String("new_value".to_string()));
    assert_eq!(result, &mut Value::String("old_value".to_string()));
}

#[test]
#[should_panic(expected = "Cannot insert into a vacant entry")]
fn test_or_insert_with_vacant_entry() {
    use serde_json::{Map, Value};

    struct MockVacantEntry<'a> {
        _phantom: std::marker::PhantomData<&'a ()>,
    }

    struct MockEntry<'a> {
        is_occupied: bool,
        vacant_entry: Option<MockVacantEntry<'a>>,
    }

    impl<'a> MockEntry<'a> {
        fn new() -> Self {
            Self {
                is_occupied: false,
                vacant_entry: None,
            }
        }

        fn or_insert_with<F>(self, default: F) -> &'a mut Value
        where
            F: FnOnce() -> Value,
        {
            if !self.is_occupied {
                panic!("Cannot insert into a vacant entry");
            } else {
                // This path is unreachable in this test
                unimplemented!()
            }
        }
    }

    let mock_entry = MockEntry::new();

    mock_entry.or_insert_with(|| Value::String("new_value".to_string()));
}

