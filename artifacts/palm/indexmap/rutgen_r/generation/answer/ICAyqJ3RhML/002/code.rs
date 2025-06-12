// Answer 0

#[derive(Debug)]
struct OccupiedEntry {
    value: i32,
}

enum Entry {
    Vacant,
    Occupied(OccupiedEntry),
}

impl Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("Entry");
        match self {
            Entry::Vacant => tuple.field(&"Vacant"),
            Entry::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

#[test]
fn test_entry_occupied_fmt() {
    let occupied_entry = OccupiedEntry { value: 42 };
    let entry = Entry::Occupied(occupied_entry);

    let result = format!("{:?}", entry);
    
    assert!(result.contains("Entry"));
    assert!(result.contains("OccupiedEntry"));
    assert!(result.contains("42"));
}

#[test]
fn test_entry_occupied_fmt_empty() {
    let occupied_entry = OccupiedEntry { value: 0 };
    let entry = Entry::Occupied(occupied_entry);

    let result = format!("{:?}", entry);
    
    assert!(result.contains("Entry"));
    assert!(result.contains("OccupiedEntry"));
    assert!(result.contains("0"));
}

