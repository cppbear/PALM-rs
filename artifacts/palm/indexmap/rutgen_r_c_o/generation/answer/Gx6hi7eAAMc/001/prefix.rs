// Answer 0

#[test]
fn test_into_mut_valid_index() {
    let mut entries = Entries::<i32, String>::new();
    entries.insert(0, String::from("value1"));
    let mut map = IndexMapCore::new();
    map.entries = entries;

    let entry = IndexedEntry::new(&mut map, 0);
    let value: &mut String = entry.into_mut();
    *value = String::from("updated_value1");
}

#[test]
#[should_panic]
fn test_into_mut_index_out_of_bounds() {
    let mut entries = Entries::<i32, String>::new();
    entries.insert(0, String::from("value1"));
    let mut map = IndexMapCore::new();
    map.entries = entries;

    let entry = IndexedEntry::new(&mut map, 1);
    let _value: &mut String = entry.into_mut(); // This should panic
}

#[test]
fn test_into_mut_multiple_entries() {
    let mut entries = Entries::<i32, String>::new();
    entries.insert(0, String::from("value1"));
    entries.insert(1, String::from("value2"));
    let mut map = IndexMapCore::new();
    map.entries = entries;

    let entry1 = IndexedEntry::new(&mut map, 0);
    let _value1: &mut String = entry1.into_mut();
    
    let entry2 = IndexedEntry::new(&mut map, 1);
    let _value2: &mut String = entry2.into_mut();
    
    // Modify the values
    *entry1.into_mut() = String::from("updated_value1");
    *entry2.into_mut() = String::from("updated_value2");
}

#[test]
fn test_into_mut_empty_entries() {
    let entries = Entries::<i32, String>::new();
    let mut map = IndexMapCore::new();
    map.entries = entries;

    // The entry creation here must be valid, but there are no entries, so this should panic.
    let entry = IndexedEntry::new(&mut map, 0);
    let _value: &mut String = entry.into_mut(); // This should panic as well
}

