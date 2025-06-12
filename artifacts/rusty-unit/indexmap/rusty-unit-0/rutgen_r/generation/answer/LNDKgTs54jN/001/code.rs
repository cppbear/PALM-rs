// Answer 0

fn test_shift_insert_new_key_at_end() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

    // Valid index for inserting a new key, which is equal to map.len()
    let result = map.shift_insert(map.len(), '+', ());
    assert_eq!(result, None);
    assert_eq!(map.get_index_of(&'+'), Some(26)); // New key '+' should be at index 26
    assert_eq!(map.len(), 28); // Map length should increase by 1
}

fn test_shift_insert_existing_key_at_invalid_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

    // This is an invalid index for moving an existing key!
    let result = std::panic::catch_unwind(|| {
        map.shift_insert(map.len(), 'a', ());
    });

    assert!(result.is_err()); // We expect a panic
}

