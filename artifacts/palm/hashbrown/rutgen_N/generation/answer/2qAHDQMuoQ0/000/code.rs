// Answer 0

#[test]
fn test_into_mut_existing_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    let value: &mut u32;
    match map.entry("poneyland") {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
    *value += 10;

    assert_eq!(map["poneyland"], 22);
}

#[test]
#[should_panic]
fn test_into_mut_non_existing_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let value: &mut u32;
    match map.entry("non_existing") {
        Entry::Occupied(_) => value = entry.into_mut(), // This line is intentionally incorrect for the test to panic.
        Entry::Vacant(_) => panic!(),
    }
}

