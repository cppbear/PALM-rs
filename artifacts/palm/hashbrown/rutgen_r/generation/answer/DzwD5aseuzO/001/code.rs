// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct CustomHasher(DefaultHasher);

    impl BuildHasher for CustomHasher {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, String, CustomHasher> = HashMap::with_hasher(CustomHasher(DefaultHasher::new()));

    let (key, value) = map.raw_entry_mut().from_key("poneyland").or_insert_with(|| {
        ("poneyland", "hoho".to_string())
    });

    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
}

#[test]
fn test_or_insert_with_multiple_calls() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct CustomHasher(DefaultHasher);

    impl BuildHasher for CustomHasher {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, String, CustomHasher> = HashMap::with_hasher(CustomHasher(DefaultHasher::new()));

    let (key1, value1) = map.raw_entry_mut().from_key("poneyland").or_insert_with(|| {
        ("poneyland", "hoho".to_string())
    });

    assert_eq!(*key1, "poneyland");
    assert_eq!(*value1, "hoho".to_string());

    let (key2, value2) = map.raw_entry_mut().from_key("unicornland").or_insert_with(|| {
        ("unicornland", "yay".to_string())
    });

    assert_eq!(*key2, "unicornland");
    assert_eq!(*value2, "yay".to_string());
}

#[should_panic(expected = "some panic condition")]
#[test]
fn test_or_insert_with_panic_condition() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct CustomHasher(DefaultHasher);

    impl BuildHasher for CustomHasher {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<&str, String, CustomHasher> = HashMap::with_hasher(CustomHasher(DefaultHasher::new()));

    // Trigger a panic condition, simulate conditions that would violate the constraints.
    let _ = map.raw_entry_mut().from_key("error_condition").or_insert_with(|| {
        panic!("some panic condition")
    });
}

