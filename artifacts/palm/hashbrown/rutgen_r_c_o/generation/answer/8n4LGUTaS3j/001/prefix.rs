// Answer 0

#[test]
fn test_and_modify_with_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table.entry(hasher(&"not_in_table"), |&(x, _)| x == "not_in_table", |(k, _)| hasher(&k));
    entry.and_modify(|(_, v)| *v += 1).or_insert(("not_in_table", 0));
}

