// Answer 0

#[test]
fn test_next_entry_seed_success() {
    struct KeySeed;
    struct ValueSeed;

    struct MapAccess {
        calls: usize,
    }

    impl MapAccess {
        fn next_key_seed(&mut self, _: KeySeed) -> Result<Option<i32>, &'static str> {
            if self.calls < 1 {
                self.calls += 1;
                Ok(Some(42)) // Returns a valid key
            } else {
                Ok(None) // No more keys
            }
        }

        fn next_value_seed(&mut self, _: ValueSeed) -> Result<i32, &'static str> {
            Ok(100) // Returns a valid value
        }
    }

    let mut map_access = MapAccess { calls: 0 };
    let result = map_access.next_entry_seed(KeySeed, ValueSeed);

    assert_eq!(result, Ok(Some((42, 100))));
}

#[test]
fn test_next_entry_seed_no_more_keys() {
    struct KeySeed;
    struct ValueSeed;

    struct MapAccess {
        calls: usize,
    }

    impl MapAccess {
        fn next_key_seed(&mut self, _: KeySeed) -> Result<Option<i32>, &'static str> {
            Ok(None) // No more keys
        }

        fn next_value_seed(&mut self, _: ValueSeed) -> Result<i32, &'static str> {
            panic!("next_value_seed should not be called when there are no keys");
        }
    }

    let mut map_access = MapAccess { calls: 0 };
    let result = map_access.next_entry_seed(KeySeed, ValueSeed);

    assert_eq!(result, Ok(None));
}

#[test]
fn test_next_entry_seed_key_error() {
    struct KeySeed;
    struct ValueSeed;

    struct MapAccess {
        calls: usize,
    }

    impl MapAccess {
        fn next_key_seed(&mut self, _: KeySeed) -> Result<Option<i32>, &'static str> {
            Err("key error") // Simulate key error
        }

        fn next_value_seed(&mut self, _: ValueSeed) -> Result<i32, &'static str> {
            Ok(100) // This should not be called
        }
    }

    let mut map_access = MapAccess { calls: 0 };
    let result = map_access.next_entry_seed(KeySeed, ValueSeed);

    assert_eq!(result, Err("key error"));
}

#[test]
fn test_next_entry_seed_value_error() {
    struct KeySeed;
    struct ValueSeed;

    struct MapAccess {
        calls: usize,
    }

    impl MapAccess {
        fn next_key_seed(&mut self, _: KeySeed) -> Result<Option<i32>, &'static str> {
            Ok(Some(42)) // Return valid key
        }

        fn next_value_seed(&mut self, _: ValueSeed) -> Result<i32, &'static str> {
            Err("value error") // Simulate value error
        }
    }

    let mut map_access = MapAccess { calls: 0 };
    let result = map_access.next_entry_seed(KeySeed, ValueSeed);

    assert_eq!(result, Err("value error"));
}

