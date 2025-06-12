// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    struct SimpleHasher {
        hash: u64,
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.hash
        }

        fn write(&mut self, bytes: &[u8]) {
            self.hash = bytes.len() as u64; // Simple implementation for testing
        }

        fn write_u32(&mut self, i: u32) {
            self.hash += i as u64;
        }

        fn write_u64(&mut self, i: u64) {
            self.hash += i;
        }

        fn write_usize(&mut self, i: usize) {
            self.hash += i as u64;
        }
    }

    let hasher = BuildHasherDefault::new();

    let mut map: HashMap<&str, String> = HashMap::with_hasher(hasher);
    
    let entry = RawEntryMut::Vacant(RawVacantEntryMut {
        table: &mut RawTable::default(),
        hash_builder: &hasher,
    });

    let (key, value) = entry.or_insert_with(|| {
        ("poneyland", "hoho".to_string())
    });

    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
}

#[test]
fn test_or_insert_with_existing_value() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("poneyland", "existing_value".to_string());

    let entry = map.raw_entry_mut().from_key("poneyland");

    match entry {
        RawEntryMut::Occupied(entry) => {
            let (key, value) = entry.into_key_value();
            assert_eq!(*key, "poneyland");
            assert_eq!(*value, "existing_value".to_string());
        },
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

