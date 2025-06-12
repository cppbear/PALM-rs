// Answer 0

#[test]
fn test_and_modify_on_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table
        .entry(
            hasher(&"poneyland"),
            |&(x, _)| x == "poneyland",
            |(k, _)| hasher(&k),
        )
        .or_insert(("poneyland", 42));

    table
        .entry(
            hasher(&"poneyland"),
            |&(x, _)| x == "poneyland",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 1)
        .or_insert(("poneyland", 42));

    assert_eq!(
        table.find(hasher(&"poneyland"), |&(k, _)| k == "poneyland"),
        Some(&("poneyland", 43))
    );
}

#[test]
fn test_and_modify_on_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table
        .entry(
            hasher(&"unicornland"),
            |&(x, _)| x == "unicornland",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 1)
        .or_insert(("unicornland", 10));

    assert_eq!(
        table.find(hasher(&"unicornland"), |&(k, _)| k == "unicornland"),
        Some(&("unicornland", 10))
    );
}

