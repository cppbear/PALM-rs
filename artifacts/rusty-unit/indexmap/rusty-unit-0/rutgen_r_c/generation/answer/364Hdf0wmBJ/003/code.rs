// Answer 0

#[test]
fn test_insert_unique_with_non_matching_keys() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<String, usize>> = Vec::new();
    let hash_value = HashValue(42);
    let key = String::from("key1");
    let value = 10;

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let occupied_entry = ref_mut.insert_unique(hash_value, key.clone(), value);

    assert_eq!(occupied_entry.get(), &value);
    assert_eq!(occupied_entry.key(), &key);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, key);
    assert_eq!(entries[0].value, value);
}

#[test]
fn test_insert_unique_should_expand_capacity() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<String, usize>> = Vec::with_capacity(1);
    let hash_value = HashValue(42);
    let key1 = String::from("key1");
    let value1 = 10;
    let key2 = String::from("key2");
    let value2 = 20;

    // Insert first entry
    {
        let mut ref_mut = RefMut::new(&mut indices, &mut entries);
        let occupied_entry = ref_mut.insert_unique(hash_value, key1.clone(), value1);
        assert_eq!(occupied_entry.get(), &value1);
    }

    // Insert second entry, should cause expansion
    {
        let mut ref_mut = RefMut::new(&mut indices, &mut entries);
        let occupied_entry = ref_mut.insert_unique(hash_value, key2.clone(), value2);
        assert_eq!(occupied_entry.get(), &value2);
    }

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[1].key, key2);
    assert_eq!(entries[1].value, value2);
}

#[test]
#[should_panic]
fn test_insert_unique_with_same_key() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<String, usize>> = Vec::new();
    let hash_value = HashValue(42);
    let key = String::from("key1");
    let value1 = 10;
    let value2 = 20;

    {
        let mut ref_mut = RefMut::new(&mut indices, &mut entries);
        ref_mut.insert_unique(hash_value, key.clone(), value1);
    }

    // This should panic if we check for duplicates within `insert_unique`
    {
        let mut ref_mut = RefMut::new(&mut indices, &mut entries);
        ref_mut.insert_unique(hash_value, key, value2);
    }
}

