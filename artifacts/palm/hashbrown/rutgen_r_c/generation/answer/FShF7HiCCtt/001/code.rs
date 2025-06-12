// Answer 0

#[test]
fn test_vacant_entry_into_value() {
    use crate::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();

    match set.entry("poneyland") {
        Entry::Occupied(_) => panic!(),
        Entry::Vacant(v) => assert_eq!(v.into_value(), "poneyland"),
    }
}

#[test]
#[should_panic]
fn test_vacant_entry_into_value_occupied() {
    use crate::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    match set.entry("poneyland") {
        Entry::Occupied(_) => {},
        Entry::Vacant(v) => assert_eq!(v.into_value(), "poneyland"),
    }
}

