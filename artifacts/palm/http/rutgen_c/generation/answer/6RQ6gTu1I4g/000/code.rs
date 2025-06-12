// Answer 0

#[test]
fn test_contains_key_empty_map() {
    struct DummyKey;

    impl AsHeaderName for DummyKey {
        fn find<K: Hash + ?Sized>(&self, _map: &HeaderMap<K>) -> Option<()>{ 
            None 
        }
    }

    let map: HeaderMap<()> = HeaderMap::with_capacity(10);
    let key = DummyKey;

    assert!(!map.contains_key(key));
}

#[test]
fn test_contains_key_with_existing_key() {
    struct DummyKey;

    impl AsHeaderName for DummyKey {
        fn find<K: Hash + ?Sized>(&self, map: &HeaderMap<K>) -> Option<()> {
            Some(())
        }
    }

    let mut map = HeaderMap::with_capacity(10);
    let key = DummyKey;

    map.insert(key.clone(), ());
    assert!(map.contains_key(key));
}

#[test]
fn test_contains_key_with_non_existing_key() {
    struct DummyKey;

    impl AsHeaderName for DummyKey {
        fn find<K: Hash + ?Sized>(&self, map: &HeaderMap<K>) -> Option<()> {
            None
        }
    }

    let mut map = HeaderMap::with_capacity(10);
    let key1 = DummyKey;
    let key2 = DummyKey;

    map.insert(key1, ());

    assert!(!map.contains_key(key2));
}

