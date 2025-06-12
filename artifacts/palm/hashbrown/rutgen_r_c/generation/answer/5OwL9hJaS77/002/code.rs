// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    // Define a simple hasher
    struct SimpleHasher(u64);
    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0 += bytes.len() as u64;
        }
    }

    struct SimpleHasherBuilder;
    impl BuildHasher for SimpleHasherBuilder {
        type Hasher = SimpleHasher;

        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher(0)
        }
    }

    // Create a HashMap with our simple hasher
    let mut map: HashMap<&str, u32, SimpleHasherBuilder> = HashMap::new();

    // Insert a key-value pair to create an occupied entry
    map.insert("occupied_key", 42);

    // Get the occupied entry
    let entry = map.raw_entry_mut().from_key("occupied_key");

    // Call or_insert and check that it returns mutable references to existing key and value
    let (key, value) = entry.or_insert("occupied_key", 100);

    // Assert that the key is the same as the existing key
    assert_eq!(*key, "occupied_key");
    
    // Assert that the value remains the same as it is occupied
    assert_eq!(*value, 42);

    // Modify the value
    *value += 10;

    // Check if the value in the map is updated correctly
    assert_eq!(map["occupied_key"], 52);
}

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};
    use std::hash::BuildHasherDefault;
    use std::hash::Hasher;

    // Define a simple hasher
    struct SimpleHasher(u64);
    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0 += bytes.len() as u64;
        }
    }

    struct SimpleHasherBuilder;
    impl BuildHasher for SimpleHasherBuilder {
        type Hasher = SimpleHasher;

        fn build_hasher(&self) -> Self::Hasher {
            SimpleHasher(0)
        }
    }

    // Create a HashMap with our simple hasher
    let mut map: HashMap<&str, u32, SimpleHasherBuilder> = HashMap::new();

    // Get a vacant entry for a nonexistent key
    let entry = map.raw_entry_mut().from_key("vacant_key");

    // Call or_insert and check that it produces a new key-value pair
    let (key, value) = entry.or_insert("vacant_key", 100);

    // Assert that the key is as expected
    assert_eq!(*key, "vacant_key");

    // Assert that the value is the newly inserted value
    assert_eq!(*value, 100);

    // Modify the value
    *value += 50;

    // Check if the value in the map is updated correctly
    assert_eq!(map["vacant_key"], 150);
}

