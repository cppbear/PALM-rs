// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use serde_json::{Map, Value};

    struct Entry<'a> {
        value: &'a mut Value,
    }

    impl<'a> Entry<'a> {
        fn into_mut(self) -> &'a mut Value {
            self.value
        }
    }

    struct OccupiedEntry<'a> {
        entry: Entry<'a>,
    }

    impl<'a> OccupiedEntry<'a> {
        fn or_insert_with<F>(self, default: F) -> &'a mut Value
        where
            F: FnOnce() -> Value,
        {
            self.entry.into_mut()
        }
    }

    let mut value = Value::String("existing_value".to_string());
    let occupied_entry = OccupiedEntry { entry: Entry { value: &mut value } };

    let result = occupied_entry.or_insert_with(|| Value::String("new_value".to_string()));

    assert_eq!(result, &mut Value::String("existing_value".to_string()));
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use serde_json::{Map, Value};

    struct Entry<'a> {
        value: Option<&'a mut Value>,
    }

    impl<'a> Entry<'a> {
        fn insert(self, value: Value) -> &'a mut Value {
            let val_ptr = Box::leak(Box::new(value));
            self.value = Some(val_ptr);
            val_ptr
        }
    }

    struct VacantEntry<'a> {
        entry: Entry<'a>,
    }

    impl<'a> VacantEntry<'a> {
        fn or_insert_with<F>(self, default: F) -> &'a mut Value
        where
            F: FnOnce() -> Value,
        {
            self.entry.insert(default())
        }
    }

    let mut entry = Entry { value: None };
    let vacant_entry = VacantEntry { entry };

    let result = vacant_entry.or_insert_with(|| Value::String("new_value".to_string()));

    assert_eq!(result, &mut Value::String("new_value".to_string()));
}

