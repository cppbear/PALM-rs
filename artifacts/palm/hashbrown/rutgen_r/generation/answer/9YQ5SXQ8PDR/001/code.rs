// Answer 0

#[test]
fn test_get_many_mut_existing_keys() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for (k, v) in [
        ("Bodleian Library", 1602),
        ("Athenæum", 1807),
        ("Herzogin-Anna-Amalia-Bibliothek", 1691),
        ("Library of Congress", 1800),
    ] {
        libraries.insert_unique(hasher(&k), (k, v), |(k, _)| hasher(&k));
    }

    let keys = ["Athenæum", "Library of Congress"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(&k)), |i, val| keys[i] == val.0);

    assert_eq!(
        got,
        [Some(&mut ("Athenæum", 1807)), Some(&mut ("Library of Congress", 1800))],
    );
}

#[test]
fn test_get_many_mut_some_missing_keys() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for (k, v) in [
        ("Athenæum", 1807),
        ("Library of Congress", 1800),
    ] {
        libraries.insert_unique(hasher(&k), (k, v), |(k, _)| hasher(&k));
    }

    let keys = ["Athenæum", "New York Public Library"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(&k)), |i, val| keys[i] == val.0);

    assert_eq!(got, [Some(&mut ("Athenæum", 1807)), None]);
}

#[should_panic]
#[test]
fn test_get_many_mut_duplicate_keys() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for (k, v) in [
        ("Athenæum", 1807),
        ("Library of Congress", 1800),
    ] {
        libraries.insert_unique(hasher(&k), (k, v), |(k, _)| hasher(&k));
    }

    let keys = ["Athenæum", "Athenæum"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(&k)), |i, val| keys[i] == val.0);
}

