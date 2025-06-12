// Answer 0

#[test]
fn entry_debug_fmt_occupied() {
    use core::fmt::Write;
    use crate::hashbrown::{HashTable, Entry, OccupiedEntry, DefaultHashBuilder};

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    table.insert("a", 1);

    match table.entry(hasher.hash_one(&"a"), |k| k == &"a", hasher) {
        Entry::Occupied(entry) => {
            let mut output = String::new();
            let result = entry.fmt(&mut output);
            assert!(result.is_ok());
            assert!(output.contains("Occupied"));
        }
        Entry::Vacant(_) => unreachable!(),
    }
}

#[test]
fn entry_debug_fmt_vacant() {
    use core::fmt::Write;
    use crate::hashbrown::{HashTable, Entry, VacantEntry, DefaultHashBuilder};

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();

    match table.entry(hasher.hash_one(&"a"), |k| k == &"a", hasher) {
        Entry::Vacant(entry) => {
            let mut output = String::new();
            let result = entry.fmt(&mut output);
            assert!(result.is_ok());
            assert!(output.contains("Vacant"));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

