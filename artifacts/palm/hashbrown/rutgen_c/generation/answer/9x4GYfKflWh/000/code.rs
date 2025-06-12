// Answer 0

#[test]
fn test_get_mut() {
    use crate::hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;
    
    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

    if let crate::hashbrown::Entry::Occupied(mut o) = table.entry(
        hasher(&"poneyland"),
        |&(x, _)| x == "poneyland",
        |(k, _)| hasher(&k),
    ) {
        o.get_mut().1 += 10;
        assert_eq!(o.get().1, 22);

        o.get_mut().1 += 2;
    }

    assert_eq!(
        table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),
        Some(&("poneyland", 24))
    );
}

#[test]
#[should_panic]
fn test_get_mut_exceed_bounds() {
    use crate::hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // This will not insert any data, just to test panic conditions
    table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

    if let crate::hashbrown::Entry::Vacant(v) = table.entry(
        hasher(&"unknown"),
        |&(x, _)| x == "unknown",
        |(k, _)| hasher(&k),
    ) {
        let _ = v.get_mut(); // This should panic as the entry is vacant
    }
}

