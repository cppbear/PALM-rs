// Answer 0

#[test]
fn test_occupied_entry_get() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&"poneyland"), "poneyland", hasher);

    match table.entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            assert_eq!(entry.get(), &"poneyland");
        }
    }
}

#[test]
fn test_occupied_entry_get_mut() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&mut str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let mut value = String::from("test");
    table.insert_unique(hasher(&value), &mut value, hasher);
    
    match table.entry(hasher(&value), |&x| x == "test", hasher) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(mut entry) => {
            *entry.get_mut() = "updated";
            assert_eq!(entry.get(), &"updated");
        }
    }
}

