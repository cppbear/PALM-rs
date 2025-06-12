// Answer 0


use std::hash::{Hash, Hasher, BuildHasher};
use hashbrown::make_hasher;

// A simple hasher implementation
#[derive(Default)]
struct SimpleHasher;
impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

#[derive(Default)]
struct SimpleBuildHasher;
impl BuildHasher for SimpleBuildHasher {
    type Hasher = SimpleHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SimpleHasher::default()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Key {
    id: u32,
}

#[test]
fn test_make_hasher() {
    let hasher_builder = SimpleBuildHasher::default();
    let hasher = make_hasher::<Key, String, SimpleBuildHasher>(&hasher_builder);

    let key1 = Key { id: 1 };
    let val1 = String::from("value1");
    
    let key2 = Key { id: 2 };
    let val2 = String::from("value2");

    let hash1 = hasher(&(key1, val1));
    let hash2 = hasher(&(key2, val2));

    assert_ne!(hash1, hash2); // Ensure different inputs yield different hashes
}

#[test]
fn test_make_hasher_with_same_key() {
    let hasher_builder = SimpleBuildHasher::default();
    let hasher = make_hasher::<Key, String, SimpleBuildHasher>(&hasher_builder);

    let key = Key { id: 1 };
    let val1 = String::from("value1");
    let val2 = String::from("value1");

    let hash1 = hasher(&(key, val1));
    let hash2 = hasher(&(key, val2));

    assert_eq!(hash1, hash2); // Ensure the same key gives the same hash
}

#[test]
#[should_panic]
fn test_make_hasher_panics_on_invalid_input() {
    let hasher_builder = SimpleBuildHasher::default();
    let hasher = make_hasher::<Key, String, SimpleBuildHasher>(&hasher_builder);

    // Assuming that our hasher implementation has a limit or invalid state
    let invalid_key = Key { id: u32::MAX }; // This might trigger an edge case
    let invalid_value = String::from("");    // Empty string, if considered invalid

    // Triggering the panic by using invalid input
    let _ = hasher(&(invalid_key, invalid_value));
}


