// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;

    struct MyEntry<'a> {
        key: &'a str,
        value: Option<u32>,
    }

    impl<'a> MyEntry<'a> {
        fn or_insert_with<F: FnOnce() -> u32>(&mut self, default: F) -> &'a mut u32 {
            match self.value {
                Some(ref mut v) => v,
                None => {
                    self.value = Some(default());
                    self.value.as_mut().unwrap()
                }
            }
        }
    }

    // Initialize a vacant entry
    let mut my_entry = MyEntry { key: "test_key", value: None };

    // Use or_insert_with with a closure that provides a default value
    let val = my_entry.or_insert_with(|| 42);
    
    // Assert that the value is properly inserted
    assert_eq!(*val, 42);
    assert_eq!(my_entry.value, Some(42));
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;

    struct MyEntry<'a> {
        key: &'a str,
        value: Option<u32>,
    }

    impl<'a> MyEntry<'a> {
        fn or_insert_with<F: FnOnce() -> u32>(&mut self, default: F) -> &'a mut u32 {
            match self.value {
                Some(ref mut v) => v,
                None => {
                    self.value = Some(default());
                    self.value.as_mut().unwrap()
                }
            }
        }
    }

    // Initialize with an occupied entry
    let mut my_entry = MyEntry { key: "test_key", value: Some(10) };

    // Use or_insert_with (which should not insert anything)
    let val = my_entry.or_insert_with(|| 42);
    
    // Assert that the original value remains unchanged
    assert_eq!(*val, 10);
    assert_eq!(my_entry.value, Some(10));
}

#[test]
fn test_or_insert_with_modify_existing_value() {
    use hashbrown::HashMap;

    struct MyEntry<'a> {
        key: &'a str,
        value: Option<u32>,
    }

    impl<'a> MyEntry<'a> {
        fn or_insert_with<F: FnOnce() -> u32>(&mut self, default: F) -> &'a mut u32 {
            match self.value {
                Some(ref mut v) => v,
                None => {
                    self.value = Some(default());
                    self.value.as_mut().unwrap()
                }
            }
        }
    }

    // Initialize an occupied entry
    let mut my_entry = MyEntry { key: "test_key", value: Some(5) };

    // Multiply the existing value by 2 using or_insert_with
    *my_entry.or_insert_with(|| 42) *= 2;

    // Assert the value has changed correctly
    assert_eq!(my_entry.value, Some(10));
}

