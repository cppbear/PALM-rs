// Answer 0

#[test]
fn test_entry_vacant_fmt() {
    use std::fmt;

    struct VacantEntry {
        value: i32,
    }

    enum Entry {
        Vacant(VacantEntry),
        Occupied(i32),
    }

    let vacant_entry = Entry::Vacant(VacantEntry { value: 42 });
    
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", vacant_entry);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Entry(42)");
}

#[test]
fn test_entry_occupied_fmt() {
    use std::fmt;

    enum Entry {
        Vacant(i32),
        Occupied(i32),
    }

    let occupied_entry = Entry::Occupied(37);
    
    let mut output = Vec::new();
    let result = write!(&mut output, "{:?}", occupied_entry);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Entry(37)");
}

