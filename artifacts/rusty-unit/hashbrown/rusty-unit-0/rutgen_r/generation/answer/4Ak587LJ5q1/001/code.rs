// Answer 0

#[derive(Debug)]
enum EntryRef<T> {
    Vacant(T),
    Occupied(T),
}

#[test]
fn test_entry_ref_vacant_display() {
    let vacant_entry = EntryRef::Vacant("test_value");

    let result = format!("{:?}", vacant_entry);
    
    assert_eq!(result, "EntryRef(\"test_value\")");
}

#[test]
fn test_entry_ref_occupied_display() {
    let occupied_entry = EntryRef::Occupied("test_value");

    let result = format!("{:?}", occupied_entry);
    
    assert_eq!(result, "EntryRef(\"test_value\")");
}

