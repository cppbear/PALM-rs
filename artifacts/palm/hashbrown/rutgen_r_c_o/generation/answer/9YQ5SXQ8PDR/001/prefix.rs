// Answer 0

#[test]
fn test_get_many_mut_valid_case() {
    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = |val: &str| val.len() as u64; // Simple hash function based on length

    libraries.insert_unique(hasher("Bodleian Library"), ("Bodleian Library", 1602), |(k, _)| hasher(k));
    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));
    libraries.insert_unique(hasher("Herzogin-Anna-Amalia-Bibliothek"), ("Herzogin-Anna-Amalia-Bibliothek", 1691), |(k, _)| hasher(k));
    libraries.insert_unique(hasher("Library of Congress"), ("Library of Congress", 1800), |(k, _)| hasher(k));

    let keys = ["Athenæum", "Library of Congress"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0);
}

#[test]
fn test_get_many_mut_missing_keys() {
    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = |val: &str| val.len() as u64;

    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));
    libraries.insert_unique(hasher("Library of Congress"), ("Library of Congress", 1800), |(k, _)| hasher(k));

    let keys = ["Athenæum", "New York Public Library"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0);
}

#[should_panic]
#[test]
fn test_get_many_mut_duplicate_keys() {
    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = |val: &str| val.len() as u64;

    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));
    libraries.insert_unique(hasher("Library of Congress"), ("Library of Congress", 1800), |(k, _)| hasher(k));

    let keys = ["Athenæum", "Athenæum"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0);
}

#[test]
fn test_get_many_mut_edge_case_empty() {
    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let keys = ["Athenæum", "Library of Congress"];
    let got = libraries.get_many_mut(keys.map(|k| k.len() as u64), |i, val| keys[i] == val.0);
}

#[test]
fn test_get_many_mut_single_key() {
    let mut libraries: HashTable<(&str, u32)> = HashTable::new();
    let hasher = |val: &str| val.len() as u64;

    libraries.insert_unique(hasher("Athenæum"), ("Athenæum", 1807), |(k, _)| hasher(k));

    let keys = ["Athenæum"];
    let got = libraries.get_many_mut(keys.map(|k| hasher(k)), |i, val| keys[i] == val.0);
}

