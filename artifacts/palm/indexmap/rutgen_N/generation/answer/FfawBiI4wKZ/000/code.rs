// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    struct TestEntry {
        value: Option<i32>,
    }

    enum Entry<'a> {
        Occupied(&'a mut i32),
        Vacant(&'a mut TestEntry),
    }

    impl<'a> Entry<'a> {
        pub fn or_insert(self, default: i32) -> &'a mut i32 {
            match self {
                Entry::Occupied(entry) => entry,
                Entry::Vacant(entry) => {
                    entry.value = Some(default);
                    entry.value.as_mut().unwrap()
                },
            }
        }
    }

    let mut vacant_entry = TestEntry { value: None };
    let entry = Entry::Vacant(&mut vacant_entry);
    
    let result = entry.or_insert(42);
    assert_eq!(*result, 42);
    assert_eq!(vacant_entry.value, Some(42));
}

#[test]
fn test_or_insert_occupied_entry() {
    struct TestEntry {
        value: i32,
    }

    enum Entry<'a> {
        Occupied(&'a mut i32),
        Vacant(&'a mut TestEntry),
    }

    impl<'a> Entry<'a> {
        pub fn or_insert(self, default: i32) -> &'a mut i32 {
            match self {
                Entry::Occupied(entry) => entry,
                Entry::Vacant(entry) => {
                    entry.value = Some(default);
                    entry.value.as_mut().unwrap()
                },
            }
        }
    }

    let mut value = 100;
    let entry = Entry::Occupied(&mut value);
    
    let result = entry.or_insert(42);
    assert_eq!(*result, 100);
    assert_eq!(value, 100);
}

