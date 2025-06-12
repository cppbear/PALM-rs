// Answer 0

#[test]
fn test_get_many_mut() {
    use hashbrown::{HashTable, Global};

    let mut libraries: HashTable<(&str, u32), Global> = HashTable::new_in(Global);

    // Simulating a simple hash function
    let hasher = |val: &&str| {
        let mut hash = 0;
        for byte in val.as_bytes() {
            hash = hash.wrapping_add(*byte as u64);
        }
        hash
    };

    // Inserting values into the hash table
    libraries.insert_unique(hasher(&"Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(&k));
    libraries.insert_unique(hasher(&"Library of Congress"), ("Library of Congress", 1800), |(k, _)| hasher(&k));

    // Test case: Getting multiple mutable references
    let keys = ["Athenæum", "Library of Congress"];
    let got = libraries.get_many_mut([hasher(&keys[0]), hasher(&keys[1])], |i, val| keys[i] == val.0);
    assert_eq!(got, [Some(&mut ("Athenæum", 1807)), Some(&mut ("Library of Congress", 1800))]);
}

#[should_panic]
#[test]
fn test_get_many_mut_duplicate_keys() {
    use hashbrown::{HashTable, Global};

    let mut libraries: HashTable<(&str, u32), Global> = HashTable::new_in(Global);

    // Simulating a simple hash function
    let hasher = |val: &&str| {
        let mut hash = 0;
        for byte in val.as_bytes() {
            hash = hash.wrapping_add(*byte as u64);
        }
        hash
    };

    // Inserting values into the hash table
    libraries.insert_unique(hasher(&"Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(&k));
    libraries.insert_unique(hasher(&"Library of Congress"), ("Library of Congress", 1800), |(k, _)| hasher(&k));

    // Test case: Duplicate keys should panic
    let keys = ["Athenæum", "Athenæum"];
    let _got = libraries.get_many_mut([hasher(&keys[0]), hasher(&keys[1])], |i, val| keys[i] == val.0);
}

