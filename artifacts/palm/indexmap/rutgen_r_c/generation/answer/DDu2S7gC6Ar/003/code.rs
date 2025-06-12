// Answer 0

#[test]
fn test_replace_full_existing_entry() {
    struct SimpleEquivalent;
    impl Equivalent<usize> for SimpleEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            true
        }
    }

    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value = HashValue(42);
    
    // Initially insert one entry
    map.replace_full(hash_value, 1, "initial".to_string());

    // Replace entry with a new value
    let (index, old_entry) = map.replace_full(hash_value, 1, "updated".to_string());

    assert_eq!(index, 0);
    assert_eq!(old_entry, Some((1, "initial".to_string())));
}

#[test]
fn test_replace_full_new_entry() {
    struct SimpleEquivalent;
    impl Equivalent<usize> for SimpleEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            true
        }
    }

    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value = HashValue(42);
    
    // Try to replace a non-existing entry
    let (index, old_entry) = map.replace_full(hash_value, 2, "new".to_string());

    assert_eq!(index, 0);
    assert_eq!(old_entry, None);
}

#[test]
#[should_panic]
fn test_replace_full_panic_on_accessing_nonexistent_entry() {
    struct SimpleEquivalent;
    impl Equivalent<usize> for SimpleEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            false
        }
    }

    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value = HashValue(42);

    // This will create a new entry
    map.replace_full(hash_value, 1, "initial".to_string());

    // Change the condition to access a nonexistent entry, which should trigger a panic
    map.replace_full(hash_value, 2, "next".to_string());
}

