// Answer 0

#[test]
fn test_get_many_unchecked_mut() {
    use hashbrown::hash_table::HashTable;
    use hashbrown::DefaultHashBuilder;
    
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
    let hashes: [u64; 2] = [hasher(&keys[0]), hasher(&keys[1])];
    
    let got = unsafe { libraries.get_many_unchecked_mut(hashes, |i, val| keys[i] == val.0) };
    assert_eq!(
        got,
        [Some(&mut ("Athenæum", 1807)), Some(&mut ("Library of Congress", 1800))],
    );

    // Test for missing key
    let keys_missing = ["Athenæum", "New York Public Library"];
    let hashes_missing: [u64; 2] = [hasher(&keys_missing[0]), hasher(&keys_missing[1])];
    
    let got_missing = unsafe { libraries.get_many_unchecked_mut(hashes_missing, |i, val| keys_missing[i] == val.0) };
    assert_eq!(got_missing, [Some(&mut ("Athenæum", 1807)), None]);
}

