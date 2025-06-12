// Answer 0

#[test]
fn test_entry_ref_key_occupied() {
    use hashbrown::HashMap;
    
    struct EntryRef<'a, K, V> {
        inner: Option<&'a (K, V)>,
    }

    impl<'a, K, V> EntryRef<'a, K, V> 
    where 
        K: std::borrow::Borrow<K> + PartialEq + std::fmt::Debug, 
        V: std::fmt::Debug,
    {
        pub fn key(&self) -> &K {
            match self.inner {
                Some((ref key, _)) => key.borrow(),
                None => panic!("Tried to access the key of a vacant entry"),
            }
        }
    }

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 3);
    
    let occupied_entry = EntryRef { inner: Some((&"poneyland".to_string(), &3)) };
    assert_eq!(occupied_entry.key(), "poneyland");
}

#[test]
#[should_panic(expected = "Tried to access the key of a vacant entry")]
fn test_entry_ref_key_vacant() {
    use hashbrown::HashMap;
    
    struct EntryRef<K, V> {
        inner: Option<(K, V)>,
    }

    impl<K, V> EntryRef<K, V> 
    where 
        K: std::borrow::Borrow<K> + std::fmt::Debug, 
        V: std::fmt::Debug,
    {
        pub fn key(&self) -> &K {
            match self.inner {
                Some((ref key, _)) => key.borrow(),
                None => panic!("Tried to access the key of a vacant entry"),
            }
        }
    }

    let vacant_entry = EntryRef { inner: None };
    vacant_entry.key();  // This will trigger a panic
}

