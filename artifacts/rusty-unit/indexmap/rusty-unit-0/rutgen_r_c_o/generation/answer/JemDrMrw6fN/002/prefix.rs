// Answer 0

#[test]
fn test_or_insert_with_occupied() {
    use std::collections::HashMap;
    use std::hash::{BuildHasher, Hasher};

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, "value1".to_string());
    let mut entries = map.clone(); // Cloning for demonstration; in real scenarios this might be different.
    let hash_builder = CustomHasher;

    let occupied = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut entries,
        index: std::collections::hash_map::OccupiedEntry::from_mut(&mut map, &1).unwrap(),
        hash_builder: PhantomData,
    });

    let _ = occupied.or_insert_with(|| (2, "value2".to_string()));
}

#[test]
fn test_or_insert_with_occupied_with_different_key() {
    use std::collections::HashMap;
    use std::hash::{BuildHasher, Hasher};

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(3, "value3".to_string());
    let mut entries = map.clone(); // Cloning for demonstration; in real scenarios this might be different.
    let hash_builder = CustomHasher;

    let occupied = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut entries,
        index: std::collections::hash_map::OccupiedEntry::from_mut(&mut map, &3).unwrap(),
        hash_builder: PhantomData,
    });

    let _ = occupied.or_insert_with(|| (4, "value4".to_string()));
}

#[test]
fn test_or_insert_with_for_multiple_entries() {
    use std::collections::HashMap;
    use std::hash::{BuildHasher, Hasher};

    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(5, "value5".to_string());
    let mut entries = map.clone(); // Cloning for demonstration; in real scenarios this might be different.
    let hash_builder = CustomHasher;

    let occupied = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut entries,
        index: std::collections::hash_map::OccupiedEntry::from_mut(&mut map, &5).unwrap(),
        hash_builder: PhantomData,
    });

    let _ = occupied.or_insert_with(|| (5, "value5_updated".to_string()));
}

