// Answer 0

#[test]
fn test_key_mut() {
    struct TestIndices;
    struct TestEntries<K, V> {
        _marker: std::marker::PhantomData<(K, V)>,
    }
    
    let mut indices = TestIndices;
    let mut entries = TestEntries { _marker: std::marker::PhantomData };
    let hash_value = HashValue(42);
    let mut vacant_entry = VacantEntry { 
        map: RefMut { indices: &mut indices, entries: &mut entries }, 
        hash: hash_value, 
        key: String::from("test_key") 
    };
    
    let key_mut: &mut String = vacant_entry.key_mut();
    *key_mut = String::from("updated_key");
    
    assert_eq!(vacant_entry.key, "updated_key");
}

#[test]
fn test_key_mut_on_different_key_type() {
    struct TestIndices;
    struct TestEntries<K, V> {
        _marker: std::marker::PhantomData<(K, V)>,
    }
    
    let mut indices = TestIndices;
    let mut entries = TestEntries { _marker: std::marker::PhantomData };
    let hash_value = HashValue(42);
    let mut vacant_entry = VacantEntry { 
        map: RefMut { indices: &mut indices, entries: &mut entries },
        hash: hash_value, 
        key: 123 
    };
    
    let key_mut: &mut i32 = vacant_entry.key_mut();
    *key_mut = 456;
    
    assert_eq!(vacant_entry.key, 456);
}

