// Answer 0

#[test]
fn test_get_many_unchecked_mut_success() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));
    libraries.insert_unique(hasher("Library of Congress"), ("Library of Congress", 1800), |(k, _)| hasher(k));

    let keys = ["Athenæum", "Library of Congress"];
    let got = unsafe { libraries.get_many_unchecked_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0) };
    assert_eq!(got, [Some(&mut ("Athenæum", 1807)), Some(&mut ("Library of Congress", 1800))]);
}

#[test]
fn test_get_many_unchecked_mut_partial_success() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));

    let keys = ["Athenæum", "New York Public Library"];
    let got = unsafe { libraries.get_many_unchecked_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0) };
    assert_eq!(got, [Some(&mut ("Athenæum", 1807)), None]);
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_overlapping_keys() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));
    
    let keys = ["Athenæum", "Athenæum"];
    let _got = unsafe { libraries.get_many_unchecked_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0) }; // Panic expected
}

