// Answer 0

#[test]
fn test_entry_ref_occupied() {
    use std::fmt;

    struct OccupiedEntry {
        value: i32,
    }

    enum EntryRef {
        Vacant(String),
        Occupied(OccupiedEntry),
    }

    impl fmt::Debug for EntryRef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                EntryRef::Vacant(ref v) => f.debug_tuple("EntryRef").field(v).finish(),
                EntryRef::Occupied(ref o) => f.debug_tuple("EntryRef").field(o).finish(),
            }
        }
    }

    let occupied_entry = OccupiedEntry { value: 42 };
    let entry_ref = EntryRef::Occupied(occupied_entry);

    let result = format!("{:?}", entry_ref);
    assert_eq!(result, "EntryRef(OccupiedEntry { value: 42 })");
}

#[test]
fn test_entry_ref_vacant() {
    use std::fmt;

    struct OccupiedEntry {
        value: i32,
    }

    enum EntryRef {
        Vacant(String),
        Occupied(OccupiedEntry),
    }

    impl fmt::Debug for EntryRef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                EntryRef::Vacant(ref v) => f.debug_tuple("EntryRef").field(v).finish(),
                EntryRef::Occupied(ref o) => f.debug_tuple("EntryRef").field(o).finish(),
            }
        }
    }

    let vacant_entry = EntryRef::Vacant("no_value".to_string());
    let result = format!("{:?}", vacant_entry);
    assert_eq!(result, "EntryRef(\"no_value\")");
}

