// Answer 0

#[test]
fn test_insert_full_vacant_case() {
    let mut index_map = IndexMapCore::<usize, String>::new();
    let hash = HashValue(123);
    let key1 = 1;
    let value1 = String::from("value1");
    index_map.insert_full(hash, key1, value1);
}

#[test]
fn test_insert_full_with_different_key() {
    let mut index_map = IndexMapCore::<usize, String>::new();
    let hash1 = HashValue(123);
    let key1 = 1;
    let value1 = String::from("value1");
    index_map.insert_full(hash1, key1, value1);

    let hash2 = HashValue(456);
    let key2 = 2;
    let value2 = String::from("value2");
    index_map.insert_full(hash2, key2, value2);
}

#[test]
fn test_insert_full_exceeding_capacity() {
    let mut index_map = IndexMapCore::<usize, String>::with_capacity(2);
    let hash1 = HashValue(1);
    let key1 = 1;
    let value1 = String::from("value1");
    index_map.insert_full(hash1, key1, value1);

    let hash2 = HashValue(2);
    let key2 = 2;
    let value2 = String::from("value2");
    index_map.insert_full(hash2, key2, value2);

    let hash3 = HashValue(3);
    let key3 = 3;
    let value3 = String::from("value3");
    index_map.insert_full(hash3, key3, value3);
}

#[test]
fn test_insert_full_when_key_collision() {
    let mut index_map = IndexMapCore::<usize, String>::new();
    let hash1 = HashValue(100);
    let key1 = 1;
    let value1 = String::from("value1");
    index_map.insert_full(hash1, key1, value1);

    let hash2 = HashValue(100); // Same hash to cause collision
    let key2 = 1; // Same key to ensure the key is not equal
    let value2 = String::from("new_value");
    index_map.insert_full(hash2, key2, value2); // This should replace the existing value with a new one
}

