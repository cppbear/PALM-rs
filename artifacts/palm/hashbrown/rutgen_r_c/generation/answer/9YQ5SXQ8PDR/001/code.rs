// Answer 0

#[test]
fn test_get_many_mut_success() {
    struct DummyAllocator;

    let mut libraries: HashTable<(&str, u32), DummyAllocator> = HashTable::new_in(DummyAllocator);
    let hasher = |val: &str| val.len() as u64; // Simple hash function based on string length
    for (k, v) in [
        ("Bodleian Library", 1602),
        ("Athenæum", 1807),
        ("Herzogin-Anna-Amalia-Bibliothek", 1691),
        ("Library of Congress", 1800),
    ] {
        libraries.insert_unique(hasher(k), (k, v), |(k, _)| hasher(k));
    }

    let keys = ["Athenæum", "Library of Congress"];
    let got: [Option<&mut (&str, u32)>; 2] =
        libraries.get_many_mut([hasher(keys[0]), hasher(keys[1])], |i, val| keys[i] == val.0);
    assert_eq!(
        got,
        [
            Some(&mut ("Athenæum", 1807)),
            Some(&mut ("Library of Congress", 1800)),
        ]
    );
}

#[test]
fn test_get_many_mut_some_missing() {
    struct DummyAllocator;

    let mut libraries: HashTable<(&str, u32), DummyAllocator> = HashTable::new_in(DummyAllocator);
    let hasher = |val: &str| val.len() as u64; // Simple hash function based on string length
    for (k, v) in [
        ("Athenæum", 1807),
        ("Library of Congress", 1800),
    ] {
        libraries.insert_unique(hasher(k), (k, v), |(k, _)| hasher(k));
    }

    let keys = ["Athenæum", "New York Public Library"];
    let got: [Option<&mut (&str, u32)>; 2] =
        libraries.get_many_mut([hasher(keys[0]), hasher(keys[1])], |i, val| keys[i] == val.0);
    assert_eq!(got, [Some(&mut ("Athenæum", 1807)), None]);
}

#[should_panic]
#[test]
fn test_get_many_mut_duplicate_keys() {
    struct DummyAllocator;

    let mut libraries: HashTable<(&str, u32), DummyAllocator> = HashTable::new_in(DummyAllocator);
    let hasher = |val: &str| val.len() as u64; // Simple hash function based on string length
    for (k, v) in [
        ("Athenæum", 1807),
        ("Library of Congress", 1800),
    ] {
        libraries.insert_unique(hasher(k), (k, v), |(k, _)| hasher(k));
    }

    // Duplicate keys should trigger a panic
    let keys = ["Athenæum", "Athenæum"];
    let _got: [Option<&mut (&str, u32)>; 2] =
        libraries.get_many_mut([hasher(keys[0]), hasher(keys[1])], |i, val| keys[i] == val.0);
}

#[test]
fn test_get_many_mut_empty() {
    struct DummyAllocator;

    let mut libraries: HashTable<(&str, u32), DummyAllocator> = HashTable::new_in(DummyAllocator);
    let hasher = |val: &str| val.len() as u64; // Simple hash function based on string length

    let keys = ["Athenæum", "Library of Congress"];
    let got: [Option<&mut (&str, u32)>; 2] =
        libraries.get_many_mut([hasher(keys[0]), hasher(keys[1])], |i, val| keys[i] == val.0);
    assert_eq!(got, [None, None]);
}

