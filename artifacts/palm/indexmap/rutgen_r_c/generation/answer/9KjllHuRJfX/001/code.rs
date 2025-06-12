// Answer 0

#[test]
fn test_swap_remove_full_entry_not_found() {
    struct MyEquivalent;

    impl Equivalent<usize> for MyEquivalent {
        fn equivalent(&self, _key: &usize) -> bool {
            false // Always return false to simulate a key not found
        }
    }

    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value = HashValue(1);

    // Attempt to swap remove a key that doesn't exist
    let result = index_map.swap_remove_full(hash_value, &MyEquivalent);
    assert_eq!(result, None);
}

