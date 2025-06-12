// Answer 0

#[test]
fn test_into_value_vacant_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();

    match set.entry("poneyland") {
        Entry::Occupied(_) => panic!(),
        Entry::Vacant(v) => {
            let value = v.into_value();
            assert_eq!(value, "poneyland");
        },
    }
}

#[test]
#[should_panic]
fn test_into_value_occupied_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    match set.entry("poneyland") {
        Entry::Occupied(_) => {
            // This should trigger the panic since the entry is occupied
            let _ = set.entry("poneyland").unwrap();
        },
        Entry::Vacant(_) => {}
    }
}

